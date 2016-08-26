use span::Spanned;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AST(pub Vec<Spanned<Item>>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub block: Spanned<Block>,
    pub item: ItemKind
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ItemKind {
    Image
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Spanned<Stmt>>, 
    pub expr: Option<Spanned<Expr>>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Assignment(String, Spanned<Expr>),
    Return(Spanned<Expr>)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(String),
    Var(String),
    Vec2(Box<(Spanned<Expr>, Spanned<Expr>)>),
    Vec3(Box<(Spanned<Expr>, Spanned<Expr>, Spanned<Expr>)>),
    BinOp(OpKind, Box<(Spanned<Expr>, Spanned<Expr>)>),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div
}

pub fn image(block: Spanned<Block>) -> Item {
    Item {
        block: block,
        item: ItemKind::Image
    }
}

pub fn block(stmts: Vec<Spanned<Stmt>>, expr: Option<Spanned<Expr>>) -> Block {
    Block {
        stmts: stmts,
        expr: expr
    }
}

pub fn ass<S: Into<String>>(name: S, expr: Spanned<Expr>) -> Stmt {
    Stmt::Assignment(name.into(), expr)
}

pub fn ret(expr: Spanned<Expr>) -> Stmt {
    Stmt::Return(expr)
}

pub fn lit<S: Into<String>>(s: S) -> Expr {
    Expr::Literal(s.into())
}

pub fn var<S: Into<String>>(s: S) -> Expr {
    Expr::Var(s.into())
}

pub fn vec2(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::Vec2(Box::new((a, b)))
}

pub fn vec3(a: Spanned<Expr>, b: Spanned<Expr>, c: Spanned<Expr>) -> Expr {
    Expr::Vec3(Box::new((a, b, c)))
}

pub fn add(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::Add, Box::new((a, b)))
}

pub fn sub(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::Sub, Box::new((a, b)))
}

pub fn mul(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::Mul, Box::new((a, b)))
}

pub fn div(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::Div, Box::new((a, b)))
}
