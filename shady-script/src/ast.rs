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
    Image,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Spanned<Stmt>>, 
    pub expr: Option<Spanned<Expr>>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Assignment(String, Spanned<Expr>),
    Return(Spanned<Expr>),
    Expr(ExprStmt),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    KeyVar(KeyVar),
    Literal(String),
    Bool(bool),
    Var(String),
    Vec2(Box<(Spanned<Expr>, Spanned<Expr>)>),
    Vec3(Box<(Spanned<Expr>, Spanned<Expr>, Spanned<Expr>)>),
    BinOp(OpKind, Box<(Spanned<Expr>, Spanned<Expr>)>),
    Stmt(ExprStmt),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum KeyVar {
    XPos,
    YPos,
    Time
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExprStmt {
    ITE(Box<(Spanned<Expr>, Spanned<Block>, Option<Spanned<Block>>)>),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OpKind {
    ArithOp(ArithOpKind),
    CmpOp(CmpOpKind),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ArithOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CmpOpKind {
    Lt,
    Gt,
    Eq,
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
    Expr::BinOp(OpKind::ArithOp(ArithOpKind::Add), Box::new((a, b)))
}

pub fn sub(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::ArithOp(ArithOpKind::Sub), Box::new((a, b)))
}

pub fn mul(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::ArithOp(ArithOpKind::Mul), Box::new((a, b)))
}

pub fn div(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::ArithOp(ArithOpKind::Div), Box::new((a, b)))
}

pub fn lt(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::CmpOp(CmpOpKind::Lt), Box::new((a, b)))
}

pub fn gt(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::CmpOp(CmpOpKind::Gt), Box::new((a, b)))
}

pub fn eq(a: Spanned<Expr>, b: Spanned<Expr>) -> Expr {
    Expr::BinOp(OpKind::CmpOp(CmpOpKind::Eq), Box::new((a, b)))
}

pub fn ite(i: Spanned<Expr>, t: Spanned<Block>, e: Option<Spanned<Block>>) -> ExprStmt {
    ExprStmt::ITE(Box::new((i, t, e)))
}

pub fn t() -> Expr {
    Expr::Bool(true)
}

pub fn f() -> Expr {
    Expr::Bool(false)
}
