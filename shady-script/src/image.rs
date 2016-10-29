use std::fmt::{self, Write};

use ::{ast, instr};

pub struct Image<'a>(&'a ::Shady, usize);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uniform {
    Time,
    MouseX,
    MouseY,
}

impl<'a> Image<'a> {
    pub fn new(shady: &'a ::Shady, idx: usize) -> Image<'a> {
        Image(shady, idx)
    }

    pub fn standalone_uniforms(&self) -> Vec<Uniform> {
        self.0.get(self.1).vars.iter().filter_map(|var| match var {
            &ast::KeyVar::Time => Some(Uniform::Time),
            &ast::KeyVar::MouseX => Some(Uniform::MouseX),
            &ast::KeyVar::MouseY => Some(Uniform::MouseY),
            _ => None
        }).collect()
    }

    pub fn standalone_shader(&self) -> String {
        let mut uniform_buffer = String::new();
        for uniform in self.standalone_uniforms().iter() {
            match *uniform {
                Uniform::Time => writeln!(uniform_buffer, "uniform float time;").unwrap(),
                Uniform::MouseX => writeln!(uniform_buffer, "uniform float mouse_x;").unwrap(),
                Uniform::MouseY => writeln!(uniform_buffer, "uniform float mouse_y;").unwrap(),
            }
        }

        let mut arg_buffer = "uv.x, uv.y".to_owned();
        for uniform in self.standalone_uniforms().iter() {
            match *uniform {
                Uniform::Time => write!(arg_buffer, ", time").unwrap(),
                Uniform::MouseX => write!(arg_buffer, ", mouse_x").unwrap(),
                Uniform::MouseY => write!(arg_buffer, ", mouse_y").unwrap(),
            }
        }

        format!(
            r#"#version 330 core

in vec2 uv;

out vec3 colour;

{}
{}

void main() {{
    colour = image({});
}}"#, 
            uniform_buffer, 
            self.0.get(self.1).shader_function(&self.standalone_uniforms()),
            arg_buffer
        )
    }
}

struct InstrVec<'a>(&'a Vec<instr::Instr>);

impl instr::Item {
    fn shader_function(&self, uniforms: &[Uniform]) -> String {
        let mut arg_buffer = "float x, float y".to_owned();
        for uniform in uniforms {
            match *uniform {
                Uniform::Time => write!(arg_buffer, ", float t").unwrap(),
                Uniform::MouseX => write!(arg_buffer, ", float mx").unwrap(),
                Uniform::MouseY => write!(arg_buffer, ", float my").unwrap()
            }
        }

        format!("vec3 image({}) {{\n{}}}", arg_buffer, InstrVec(&self.instrs))
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
            &instr::ExprKind::KeyVar(ast::KeyVar::XPos) => write!(f, "x"),
            &instr::ExprKind::KeyVar(ast::KeyVar::YPos) => write!(f, "y"),
            &instr::ExprKind::KeyVar(ast::KeyVar::Time) => write!(f, "t"),
            &instr::ExprKind::KeyVar(ast::KeyVar::MouseX) => write!(f, "mx"),
            &instr::ExprKind::KeyVar(ast::KeyVar::MouseY) => write!(f, "my"),
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

