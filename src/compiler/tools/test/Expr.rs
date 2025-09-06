use crate::compiler::token::Token;
use crate::compiler::expr::Object;

#[derive(Debug, Clone)]
pub enum expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Ternary(Box<Ternary>),
}

pub trait exprVisitor<T> {
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
    fn visit_ternary(&self, ternary: &Ternary) -> T;
}

impl expr {
    pub fn accept<T>(&self, visitor: &dyn exprVisitor<T>) -> T {
        match self {
            expr::Binary(b) => visitor.visit_binary(b),
            expr::Grouping(b) => visitor.visit_grouping(b),
            expr::Literal(b) => visitor.visit_literal(b),
            expr::Unary(b) => visitor.visit_unary(b),
            expr::Ternary(b) => visitor.visit_ternary(b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub true_branch: Box<Expr>,
    pub false_branch: Box<Expr>,
}

