use crate::scanner::token::Token;
use crate::scanner::token_type;

#[derive(Clone, Debug)]
pub enum Expr<'expr> {
    Assign(Assign<'expr>),
    Binary(Binary<'expr>),
    Call(Call<'expr>),
    Get(Get<'expr>),
    Grouping(GroupingExpr<'expr>),
    Literal(LiteralExpr<'expr>),
    Set(Set<'expr>),
    Super(Super<'expr>),
    This(This<'expr>),
    Unary(Unary<'expr>),
    Variable(Variable<'expr>),
}

#[derive(Clone, Debug)]
pub struct Assign<'expr> {
    pub name: Token<'expr>,
    pub value: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Binary<'expr> {
    pub left: Box<Expr<'expr>>,
    pub operator: Token<'expr>,
    pub right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Call<'expr> {
    pub callee: Box<Expr<'expr>>,
    pub parenthesis: Token<'expr>,
    pub arguments: Vec<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Get<'expr> {
    pub object: Box<Expr<'expr>>,
    pub name: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct GroupingExpr<'expr> {
    pub expression: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct LiteralExpr<'expr> {
    pub value: token_type::Literal<'expr>,
}

#[derive(Clone, Debug)]
pub struct Logical<'expr> {
    pub left: Box<Expr<'expr>>,
    pub operator: Token<'expr>,
    pub right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Set<'expr> {
    pub object: Box<Expr<'expr>>,
    pub name: Token<'expr>,
    pub value: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Super<'expr> {
    pub keyword: Token<'expr>,
    pub method: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct This<'expr> {
    pub keyword: Token<'expr>,
}

#[derive(Clone, Debug)]
pub struct Unary<'expr> {
    pub operator: Token<'expr>,
    pub right: Box<Expr<'expr>>,
}

#[derive(Clone, Debug)]
pub struct Variable<'expr> {
    pub name: Token<'expr>,
}
