use ast;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub ret: Option<Type>,
    pub instrs: Vec<Instr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub ret: Type,
    pub kind: ast::ItemKind,
    pub instrs: Vec<Instr>,
    pub vars: BTreeSet<ast::KeyVar>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instr {
    Decl(String, Type, Option<ExprKind>),
    Assignment(String, Expr),
    Return(Expr),
    ITE(ExprKind, Block, Option<Block>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expr {
    pub ty: Type,
    pub expr: ExprKind
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExprKind {
    KeyVar(ast::KeyVar),
    Literal(String),
    Bool(bool),
    Var(String),
    Vec2(Box<(ExprKind, ExprKind)>),
    Vec3(Box<(ExprKind, ExprKind, ExprKind)>),
    BinOp(ast::OpKind, Box<(ExprKind, ExprKind)>),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Void,
    Float,
    Bool,
    Vec2,
    Vec3
}
