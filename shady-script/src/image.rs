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

struct InstrVec<'a>(&'a Vec<instr::Instr>);

impl instr::Item {
    fn shader_function(&self) -> String {
        format!("vec3 image(float x, float y) {{\n{}}}", InstrVec(&self.instrs))
    }
}

impl fmt::Display for instr::Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for inst in &self.instrs {
            try!(write!(f, "    {};\n", inst))
        }

        Ok(())
    }
}

impl<'a> fmt::Display for InstrVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for inst in self.0 {
            try!(write!(f, "    {};\n", inst))
        }

        Ok(())
    }
}

impl fmt::Display for instr::Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &instr::Instr::Decl(ref name, ref ty, ref expr) => if let &Some(ref expr) = expr {
                write!(f, "{} {} = {}", ty, name, expr)
            } else {
                write!(f, "{} {}", ty, name)
            },
            &instr::Instr::Assignment(ref name, ref expr) => write!(f, "{} = {}", name, expr.expr),
            &instr::Instr::Return(ref expr) => write!(f, "return {}", expr.expr),
            &instr::Instr::ITE(ref expr, ref block, None) => write!(f, "if {} {{\n{}}}", expr, block),
            &instr::Instr::ITE(ref expr, ref tblock, Some(ref eblock)) => write!(f, "if({}) {{\n{}}} else {{\n{}}}", expr, tblock, eblock),
        }
    }
}

impl fmt::Display for instr::Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &instr::Type::Void => write!(f, "void"),
            &instr::Type::Bool => write!(f, "bool"),
            &instr::Type::Float => write!(f, "float"),
            &instr::Type::Vec2 => write!(f, "vec2"),
            &instr::Type::Vec3 => write!(f, "vec3"),
        }
    }
}

impl fmt::Display for instr::ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &instr::ExprKind::Literal(ref s) => write!(f, "{}", s),
            &instr::ExprKind::Bool(ref b) => write!(f, "{}", b),
            &instr::ExprKind::Var(ref s) => write!(f, "{}", s),
            &instr::ExprKind::Vec2(ref exprs) => write!(f, "vec2({}, {})", exprs.0, exprs.1),
            &instr::ExprKind::Vec3(ref exprs) => write!(f, "vec3({}, {}, {})", exprs.0, exprs.1, exprs.2),
            &instr::ExprKind::BinOp(ast::OpKind::ArithOp(ast::ArithOpKind::Add), ref exprs) => write!(f, "{} + {}", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::ArithOp(ast::ArithOpKind::Sub), ref exprs) => write!(f, "{} - {}", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::ArithOp(ast::ArithOpKind::Mul), ref exprs) => write!(f, "({}) * ({})", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::ArithOp(ast::ArithOpKind::Div), ref exprs) => write!(f, "({}) / ({})", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::CmpOp(ast::CmpOpKind::Lt), ref exprs) => write!(f, "({}) < ({})", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::CmpOp(ast::CmpOpKind::Gt), ref exprs) => write!(f, "({}) > ({})", exprs.0, exprs.1),
            &instr::ExprKind::BinOp(ast::OpKind::CmpOp(ast::CmpOpKind::Eq), ref exprs) => write!(f, "({}) == ({})", exprs.0, exprs.1),
        }
    }
}

