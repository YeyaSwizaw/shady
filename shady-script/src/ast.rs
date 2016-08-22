#[derive(Debug, Eq, PartialEq)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug, Eq, PartialEq)]
pub enum Stmt {
    Return(Expr)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Literal(String),
    Vec2(Box<[Expr; 2]>),
    Vec3(Box<[Expr; 3]>),
}

pub fn ret(expr: Expr) -> Stmt {
    Stmt::Return(expr)
}

pub fn lit<S: Into<String>>(s: S) -> Expr {
    Expr::Literal(s.into())
}

pub fn vec2(a: Expr, b: Expr) -> Expr {
    Expr::Vec2(Box::new([a, b]))
}

pub fn vec3(a: Expr, b: Expr, c: Expr) -> Expr {
    Expr::Vec3(Box::new([a, b, c]))
}
