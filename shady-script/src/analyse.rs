use ast;
use instr;
use span::{Span, Spanned};

use std::collections::{HashMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnalyseError {
    IncorrectReturnType(Span),
    IncorrectTupleTypes(Span),
    IncorrectBinOpTypes(Span),
    IncorrectAssignmentType(Span),
    UndefinedName(Span),
    ExpectedReturn(Span),
    ExpectedBoolean(Span),
    ExpectedVoidExprStmt(Span)
}

struct Env {
    names: HashMap<String, instr::Type>,
    used: BTreeSet<ast::KeyVar>,
}

impl Env {
    fn new() -> Env {
        Env {
            names: HashMap::new(),
            used: BTreeSet::new()
        }
    }

    fn lookup(&self, name: &str) -> Option<instr::Type> {
        self.names.get(name).cloned()
    }

    fn insert<S: Into<String>>(&mut self, name: S, ty: instr::Type) {
        self.names.insert(name.into(), ty);
    }

    fn use_var(&mut self, var: ast::KeyVar) {
        self.used.insert(var);
    }
}

impl ast::AST {
    pub fn analyse(&self) -> Result<::Shady, AnalyseError> {
        let mut shady = ::Shady::new();

        for item in &self.0 {
            shady.push_item(try!(analyse_item(&item)));
        }

        Ok(shady)
    }
}

fn analyse_item(item: &Spanned<ast::Item>) -> Result<instr::Item, AnalyseError> {
    let mut env = Env::new();

    let block = try!(analyse_block(&mut env, &item.data.block, Some(&mut |block, env, expr| {
        let e = try!(analyse_expr(env, expr));

        if let Some(ty) = block.ret {
            if ty != e.ty {
                return Err(AnalyseError::IncorrectReturnType(expr.span));
            }
        };

        println!("{:?}", block);
        println!("{:?}", e);

        block.ret = Some(e.ty);
        block.instrs.push(instr::Instr::Return(e));
        Ok(())
    })));

    match item.data.item {
        ast::ItemKind::Image => if let Some(instr::Type::Vec3) = block.ret {
            Ok(instr::Item {
                ret: instr::Type::Vec3,
                kind: ast::ItemKind::Image,
                instrs: block.instrs,
                vars: env.used
            })
        } else {
            unimplemented!();
        },
    }
}

fn analyse_block(env: &mut Env, block: &Spanned<ast::Block>, expr_handler: Option<&mut FnMut(&mut instr::Block, &mut Env, &Spanned<ast::Expr>) -> Result<(), AnalyseError>>) -> Result<instr::Block, AnalyseError> {
    let mut stmts = Vec::new();
    let mut ret = None;

    for stmt in &block.data.stmts {
        match stmt.data {
            ast::Stmt::Assignment(ref name, ref expr) => {
                let expr = try!(analyse_expr(env, expr));

                match env.lookup(name) {
                    Some(ty) => if expr.ty != ty {
                        return Err(AnalyseError::IncorrectAssignmentType(stmt.span))
                    } else {
                        stmts.push(instr::Instr::Assignment(name.clone(), expr));
                    },

                    None => {
                        env.insert(name.clone(), expr.ty);
                        stmts.push(instr::Instr::Decl(name.clone(), expr.ty, Some(expr.expr)))
                    }
                }
            },

            ast::Stmt::Return(ref expr) => {
                let expr = try!(analyse_expr(env, expr));

                if let Some(ty) = ret {
                    if expr.ty != ty {
                        return Err(AnalyseError::IncorrectReturnType(stmt.span));
                    }
                } else {
                    ret = Some(expr.ty)
                };

                stmts.push(instr::Instr::Return(expr));
            },

            ast::Stmt::Expr(ast::ExprStmt::ITE(ref exprs)) => {
                let i = try!(analyse_expr(env, &exprs.0));
                if i.ty != instr::Type::Bool {
                    return Err(AnalyseError::ExpectedBoolean(exprs.0.span));
                }

                // TODO: Parent environment!!!
                let t = try!(analyse_block(env, &exprs.1, None));
                if let Some(ety) = t.ret {
                    if let Some(ty) = ret {
                        if ty != ety {
                            return Err(AnalyseError::IncorrectReturnType(exprs.1.span));
                        }
                    } else {
                        ret = t.ret
                    }
                }

                let e = if let Some(ref b) = exprs.2 {
                    // TODO: Parent environment!!!
                    let e = try!(analyse_block(env, &b, None));
                    if let Some(ety) = e.ret {
                        if let Some(ty) = ret {
                            if ty != ety {
                                return Err(AnalyseError::IncorrectReturnType(b.span));
                            }
                        } else {
                            ret = e.ret
                        }
                    }

                    Some(e)
                } else {
                    None
                };

                stmts.push(instr::Instr::ITE(i.expr, t, e));
            },
        }
    }

    let mut b = instr::Block {
        ret: ret,
        instrs: stmts,
    };

    if let Some(ref expr) = block.data.expr {
        if let Some(handler) = expr_handler {
            try!(handler(&mut b, env, expr))
        } else {
            return Err(AnalyseError::ExpectedVoidExprStmt(expr.span))
        }
    };

    Ok(b)
}

fn analyse_expr(env: &mut Env, expr: &Spanned<ast::Expr>) -> Result<instr::Expr, AnalyseError> {
    match expr.data {
        ast::Expr::KeyVar(var) => {
            env.use_var(var);

            Ok(instr::Expr {
                ty: instr::Type::Float,
                expr: instr::ExprKind::KeyVar(var)
            })
        },

        ast::Expr::Literal(ref lit) => Ok(instr::Expr {
            ty: instr::Type::Float,
            expr: instr::ExprKind::Literal(lit.clone())
        }),

        ast::Expr::Bool(b) => Ok(instr::Expr {
            ty: instr::Type::Bool,
            expr: instr::ExprKind::Bool(b)
        }),

        ast::Expr::Var(ref name) => env.lookup(name)
            .ok_or(AnalyseError::UndefinedName(expr.span))
            .map(|ty| instr::Expr {
                ty: ty,
                expr: instr::ExprKind::Var(name.clone())
            }),

        ast::Expr::Vec2(ref exprs) => {
            let e1 = try!(analyse_expr(env, &exprs.0));
            let e2 = try!(analyse_expr(env, &exprs.1));

            if e1.ty != e2.ty {
                Err(AnalyseError::IncorrectTupleTypes(expr.span))
            } else {
                Ok(instr::Expr {
                    ty: instr::Type::Vec2,
                    expr: instr::ExprKind::Vec2(Box::new((e1.expr, e2.expr)))
                })
            }
        },

        ast::Expr::Vec3(ref exprs) => {
            let e1 = try!(analyse_expr(env, &exprs.0));
            let e2 = try!(analyse_expr(env, &exprs.1));
            let e3 = try!(analyse_expr(env, &exprs.2));

            if e1.ty != e2.ty || e1.ty != e3.ty {
                Err(AnalyseError::IncorrectTupleTypes(expr.span))
            } else {
                Ok(instr::Expr {
                    ty: instr::Type::Vec3,
                    expr: instr::ExprKind::Vec3(Box::new((e1.expr, e2.expr, e3.expr)))
                })
            }
        },

        ast::Expr::BinOp(op @ ast::OpKind::ArithOp(_), ref exprs) => {
            let e1 = try!(analyse_expr(env, &exprs.0));
            let e2 = try!(analyse_expr(env, &exprs.1));

            match (e1.ty, e2.ty) {
                (instr::Type::Float, ty) | (ty, instr::Type::Float) => Ok(instr::Expr {
                    ty: ty,
                    expr: instr::ExprKind::BinOp(op, Box::new((e1.expr, e2.expr)))
                }),

                (ref t, ref u) if *t == *u => Ok(instr::Expr {
                    ty: t.clone(),
                    expr: instr::ExprKind::BinOp(op, Box::new((e1.expr, e2.expr)))
                }),

                _ => Err(AnalyseError::IncorrectBinOpTypes(expr.span))
            }
        },

        ast::Expr::BinOp(op @ ast::OpKind::CmpOp(_), ref exprs) => {
            let e1 = try!(analyse_expr(env, &exprs.0));
            let e2 = try!(analyse_expr(env, &exprs.1));

            if e1.ty == e2.ty {
                Ok(instr::Expr {
                    ty: instr::Type::Bool,
                    expr: instr::ExprKind::BinOp(op, Box::new((e1.expr, e2.expr)))
                })
            } else {
                Err(AnalyseError::IncorrectBinOpTypes(expr.span))
            }
        },

        ast::Expr::Stmt(_) => unimplemented!()
    }
}

/*

    fn expr_type(&self, expr: &ast::Expr) -> Result<instr::Type, AnalyseError> {
        match expr {
            &ast::Expr::Literal(_) => Ok(instr::Type::Float),
            &ast::Expr::Bool(_) => Ok(instr::Type::Bool),

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

            &ast::Expr::BinOp(ast::OpKind::ArithOp(_), ref exprs) => {
                match (try!(self.expr_type(&exprs.0.data)), try!(self.expr_type(&exprs.1.data))) {
                    (instr::Type::Float, ty) | (ty, instr::Type::Float) => Ok(ty),
                    (ref a, ref b) if *a == *b => Ok(*a),
                    _ => Err(AnalyseError::IncorrectBinOpTypes(Span { begin: exprs.0.span.begin, end: exprs.1.span.end }))
                }
            },

            &ast::Expr::BinOp(ast::OpKind::CmpOp(_), ref exprs) => {
                match (try!(self.expr_type(&exprs.0.data)), try!(self.expr_type(&exprs.1.data))) {
                    (ref a, ref b) if *a == *b => Ok(*a),
                    _ => Err(AnalyseError::IncorrectBinOpTypes(Span { begin: exprs.0.span.begin, end: exprs.1.span.end }))
                }
            },

            &ast::Expr::Stmt(ast::ExprStmt::ITE(ref ite)) => {

            }
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

*/
