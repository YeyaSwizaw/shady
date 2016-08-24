#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AST(pub Vec<Item>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub block: Block,
    pub item: ItemKind
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ItemKind {
    Image
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Return(Expr)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(String),
    Vec2(Box<(Expr, Expr)>),
    Vec3(Box<(Expr, Expr, Expr)>)
}

pub fn image(block: Block) -> Item {
    Item {
        block: block,
        item: ItemKind::Image
    }
}

pub fn ret(expr: Expr) -> Stmt {
    Stmt::Return(expr)
}

pub fn lit<S: Into<String>>(s: S) -> Expr {
    Expr::Literal(s.into())
}

pub fn vec2(a: Expr, b: Expr) -> Expr {
    Expr::Vec2(Box::new((a, b)))
}

pub fn vec3(a: Expr, b: Expr, c: Expr) -> Expr {
    Expr::Vec3(Box::new((a, b, c)))
}
