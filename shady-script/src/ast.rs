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
pub struct Block(pub Vec<Spanned<Stmt>>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Return(Spanned<Expr>)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(String),
    Var(String),
    Vec2(Box<(Spanned<Expr>, Spanned<Expr>)>),
    Vec3(Box<(Spanned<Expr>, Spanned<Expr>, Spanned<Expr>)>)
}

pub fn image(block: Spanned<Block>) -> Item {
    Item {
        block: block,
        item: ItemKind::Image
    }
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
