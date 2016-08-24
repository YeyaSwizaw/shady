use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Span {
    pub begin: usize,
    pub end: usize
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spanned<T: Debug + Eq + PartialEq + Clone> {
    pub span: Span,
    pub data: T
}

pub fn spanned<T: Debug + Eq + PartialEq + Clone>(begin: usize, end: usize, data: T) -> Spanned<T> {
    Spanned { 
        span: Span {
            begin: begin,
            end: end
        },
        data: data
    }
}

