use crate::token::{token_type, Token};

#[derive(Clone, Debug)]
pub enum Expr<'expr> {
    Assign(Assign<'expr>),
    Binary(Binary<'expr>),
    Call(Call<'expr>),
    Get(Get<'expr>),
    Grouping(Grouping<'expr>),
    Literal(Literal<'expr>),
    Set(Set<'expr>),
    Super(Super<'expr>),
    This(This<'expr>),
    Unary(Unary<'expr>),
    Variable(Variable<'expr>),
}

#[derive(Clone, Debug)]
pub struct Assign<'expr> {
    name: Token<'expr>,
    value: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Binary<'expr> {
    left: Box<Expr<'expr>>,
    operator: Token<'expr>,
    right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Call<'expr> {
    callee: Box<Expr<'expr>>,
    parenthesis: Token<'expr>,
    arguments: Vec<Token<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Get<'expr> {
    object: Box<Expr<'expr>>,
    name: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct Grouping<'expr> {
    expression: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Literal<'expr> {
    value: token_type::Literal<'expr>,
}

#[derive(Clone, Debug)]
pub struct Logical<'expr> {
    left: Box<Expr<'expr>>,
    operator: Token<'expr>,
    right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Set<'expr> {
    object: Box<Expr<'expr>>,
    name: Token<'expr>,
    value: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Super<'expr> {
    keyword: Token<'expr>,
    method: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct This<'expr> {
    keyword: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct Unary<'expr> {
    operator: Token<'expr>,
    right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Variable<'expr> {
    name: Token<'expr>,
}
