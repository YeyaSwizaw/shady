use ast;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block(pub Vec<Instr>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub ret: Type,
    pub kind: ast::ItemKind,
    pub instrs: Block
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instr {
    Return(ast::Expr)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Float,
    Vec2,
    Vec3
}

pub fn ret(expr: ast::Expr) -> Instr {
    Instr::Return(expr)
}
