use ast;
use instr;

impl ast::AST {
    pub fn analyse(&self) -> ::Shady {
        let mut shady = ::Shady::new();

        for item in &self.0 {
            shady.push_item(item.analyse());
        }

        shady
    }
}

impl ast::Item {
    pub fn analyse(&self) -> instr::Item {
        let mut item = instr::Item::new(self.item);

        for stmt in &self.block.0 {
            match stmt {
                &ast::Stmt::Return(ref expr) => if item.expr_type(expr) == item.ret {
                    item.push_instr(instr::ret(expr.clone()));
                } else {
                    panic!("Expected return type")
                }
            }
        }

        item
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

    fn expr_type(&self, expr: &ast::Expr) -> instr::Type {
        match expr {
            &ast::Expr::Literal(_) => instr::Type::Float,

            &ast::Expr::Vec2(ref exprs) => {
                let ty = self.expr_type(&exprs.0);
                if self.expr_type(&exprs.1) == ty {
                    instr::Type::Vec2
                } else {
                    panic!("Tuple must be same type")
                }
            },

            &ast::Expr::Vec3(ref exprs) => {
                let ty = self.expr_type(&exprs.0);
                if self.expr_type(&exprs.1) == ty && self.expr_type(&exprs.2) == ty {
                    instr::Type::Vec3
                } else {
                    panic!("Tuple must be same type")
                }
            },
        }
    }

    fn push_instr(&mut self, instr: instr::Instr) {
        self.instrs.0.push(instr)
    }
}
