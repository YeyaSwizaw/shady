use ast;
use instr;
use span::Span;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnalyseError {
    IncorrectReturnType(Span),
    IncorrectTupleTypes(Span),
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

        for stmt in &self.block.data.0 {
            match &stmt.data {
                &ast::Stmt::Return(ref expr) => if try!(item.expr_type(&expr.data)) == item.ret {
                    item.push_instr(instr::ret(expr.data.clone()));
                } else {
                    return Err(AnalyseError::IncorrectReturnType(stmt.span));
                }
            }
        }

        Ok(item)
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

    fn expr_type(&self, expr: &ast::Expr) -> Result<instr::Type, AnalyseError> {
        match expr {
            &ast::Expr::Literal(_) => Ok(instr::Type::Float),

            &ast::Expr::Var(_) => Ok(instr::Type::Float),

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
        }
    }

    fn push_instr(&mut self, instr: instr::Instr) {
        self.instrs.0.push(instr)
    }
}
