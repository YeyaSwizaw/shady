use std::fmt;

use ::{ast, instr};

pub struct Image<'a>(&'a ::Shady, usize);

impl<'a> Image<'a> {
    pub fn new(shady: &'a ::Shady, idx: usize) -> Image<'a> {
        Image(shady, idx)
    }

    pub fn standalone_shader(&self) -> String {
        format!(
            r#"#version 330 core

in vec2 uv;

out vec3 colour;

{}

void main() {{
    colour = image(uv.x, uv.y);
}}"#, 
            self.0.get(self.1).shader_function()
        )
    }
}

impl instr::Item {
    fn shader_function(&self) -> String {
        format!("vec3 image(float x, float y) {{\n {}}}", self.instrs)
    }
}

impl fmt::Display for instr::Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for inst in self.0.iter().rev() {
            try!(write!(f, "    {};\n", inst))
        }

        Ok(())
    }
}

impl fmt::Display for instr::Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &instr::Instr::Return(ref expr) => write!(f, "return {}", expr)
        }
    }
}

impl fmt::Display for ast::Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ast::Expr::Literal(ref s) => write!(f, "{}", s),
            &ast::Expr::Var(ref s) => write!(f, "{}", s),
            &ast::Expr::Vec2(ref exprs) => write!(f, "vec2({}, {})", exprs.0.data, exprs.1.data),
            &ast::Expr::Vec3(ref exprs) => write!(f, "vec3({}, {}, {})", exprs.0.data, exprs.1.data, exprs.2.data),
            &ast::Expr::BinOp(ast::OpKind::Add, ref exprs) => write!(f, "{} + {}", exprs.0.data, exprs.1.data),
            &ast::Expr::BinOp(ast::OpKind::Sub, ref exprs) => write!(f, "{} - {}", exprs.0.data, exprs.1.data),
            &ast::Expr::BinOp(ast::OpKind::Mul, ref exprs) => write!(f, "({}) * ({})", exprs.0.data, exprs.1.data),
            &ast::Expr::BinOp(ast::OpKind::Div, ref exprs) => write!(f, "({}) / ({})", exprs.0.data, exprs.1.data),
        }
    }
}

