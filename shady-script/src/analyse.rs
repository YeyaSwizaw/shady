use ast;
use instr;
use span::Span;

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnalyseError {
    IncorrectReturnType(Span),
    IncorrectTupleTypes(Span),
    IncorrectBinOpTypes(Span),
    UndefinedName(String),
    ExpectedReturn(Span)
}

struct Env {
    names: HashMap<String, instr::Type>
}

impl Env {
    fn new(item: ast::ItemKind) -> Env {
        let mut names = HashMap::new();

        match item {
            ast::ItemKind::Image => {
                names.insert("x".into(), instr::Type::Float);
                names.insert("y".into(), instr::Type::Float);
            }
        }

        Env {
            names: names
        }
    }

    fn expr_type(&self, expr: &ast::Expr) -> Result<instr::Type, AnalyseError> {
        match expr {
            &ast::Expr::Literal(_) => Ok(instr::Type::Float),

            &ast::Expr::Var(ref name) => self.lookup(name).ok_or(AnalyseError::UndefinedName(name.clone())),

            &ast::Expr::Vec2(ref exprs) => {
                let ty = try!(self.expr_type(&exprs.0.data));
                if try!(self.expr_type(&exprs.1.data)) == ty {
                    Ok(instr::Type::Vec2)
                } else {
                    Err(AnalyseError::IncorrectTupleTypes(Span { begin: exprs.0.span.begin, end: exprs.1.span.end }))
                }
            },

            &ast::Expr::Vec3(ref exprs) => {
                let ty = try!(self.expr_type(&exprs.0.data));
                if try!(self.expr_type(&exprs.1.data)) == ty && try!(self.expr_type(&exprs.2.data)) == ty {
                    Ok(instr::Type::Vec3)
                } else {
                    Err(AnalyseError::IncorrectTupleTypes(Span { begin: exprs.0.span.begin, end: exprs.2.span.end }))
                }
            },

            &ast::Expr::BinOp(_, ref exprs) => {
                match (try!(self.expr_type(&exprs.0.data)), try!(self.expr_type(&exprs.1.data))) {
                    (instr::Type::Float, ty) | (ty, instr::Type::Float) => Ok(ty),
                    (ref a, ref b) if *a == *b => Ok(*a),
                    _ => Err(AnalyseError::IncorrectBinOpTypes(Span { begin: exprs.0.span.begin, end: exprs.1.span.end }))
                }
            },
        }
    }


    fn lookup(&self, name: &str) -> Option<instr::Type> {
        self.names.get(name).cloned()
    }

    fn insert<S: Into<String>>(&mut self, name: S, ty: instr::Type) {
        self.names.insert(name.into(), ty);
    }
}

impl ast::AST {
    pub fn analyse(&self) -> Result<::Shady, AnalyseError> {
        let mut shady = ::Shady::new();

        for item in &self.0 {
            shady.push_item(try!(item.data.analyse()));
        }

        Ok(shady)
    }
}

impl ast::Item {
    pub fn analyse(&self) -> Result<instr::Item, AnalyseError> {
        let mut item = instr::Item::new(self.item);
        let mut env = Env::new(self.item);

        for stmt in &self.block.data.stmts {
            match &stmt.data {
                &ast::Stmt::Assignment(ref name, ref expr) => {
                    let ty = try!(env.expr_type(&expr.data));
                    env.insert(name.clone(), ty);
                    item.push_instr(instr::ass(ty, name.clone(), expr.data.clone()));
                },

                &ast::Stmt::Return(ref expr) => if try!(env.expr_type(&expr.data)) == item.ret {
                    item.push_instr(instr::ret(expr.data.clone()));
                } else {
                    return Err(AnalyseError::IncorrectReturnType(stmt.span));
                }
            }
        }

        if let Some(ref expr) = self.block.data.expr {
            if try!(env.expr_type(&expr.data)) == item.ret {
                item.push_instr(instr::ret(expr.data.clone()));
            } else {
                return Err(AnalyseError::IncorrectReturnType(expr.span));
            }
        };

        if let Some(&instr::Instr::Return(_)) = item.instrs.0.last() {
            Ok(item)
        } else {
            return Err(AnalyseError::ExpectedReturn(self.block.span))
        }

    }
}

impl instr::Item {
    fn new(kind: ast::ItemKind) -> instr::Item {
        match kind {
            ast::ItemKind::Image => instr::Item {
                ret: instr::Type::Vec3,
                kind: ast::ItemKind::Image,
                instrs: instr::Block(Vec::new())
            }
        }
    }

    fn push_instr(&mut self, instr: instr::Instr) {
        self.instrs.0.push(instr)
    }
}
