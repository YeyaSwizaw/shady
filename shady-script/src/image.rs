use ::ast;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Eq, PartialEq)]
pub struct ImageDefinition(pub ast::Block);

impl ImageDefinition {
    pub fn generate_shader_function(&self) -> String {
        format!("vec3 image(float x, float y) {{\n{}\n}}", self.0)
    }

    pub fn generate_fragment_shader(&self) -> String {
        format!(
r#"#version 330 core

in vec2 uv;

out vec3 colour;

{}

void main() {{
    colour = image(uv.x, uv.y);
}}"#,
            self.generate_shader_function()
        )
    }
}

impl Display for ast::Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for stmt in &self.0 {
            try!(write!(f, "{};", stmt));
        }

        Ok(())
    }
}

impl Display for ast::Stmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ast::Stmt::Return(ref expr) => write!(f, "    return {}", expr)
        }
    }
}

impl Display for ast::Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ast::Expr::Literal(ref s) => write!(f, "{}", s),
            &ast::Expr::Vec2(ref exprs) => write!(f, "vec2({}, {})", exprs[0], exprs[1]),
            &ast::Expr::Vec3(ref exprs) => write!(f, "vec3({}, {}, {})", exprs[0], exprs[1], exprs[2])
        }
    }
}

#[test]
fn test_image_def() {
    let src = r#"
        image() {
            return (12, 23, 34)
        }
    "#;

    assert_eq!(::grammar::parse_ImageDefinition(src).unwrap().generate_shader_function(),
r#"vec3 image(float x, float y) {
    return vec3(12, 23, 34);
}"#);
}
