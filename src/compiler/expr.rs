use crate::compiler::token::Token;

// Define Object type to represent Lox values
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

// Enum-based AST representation
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
    Ternary(Box<Ternary>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(b) => visitor.visit_binary(b),
            Expr::Grouping(g) => visitor.visit_grouping(g),
            Expr::Literal(l) => visitor.visit_literal(l),
            Expr::Unary(u) => visitor.visit_unary(u),
            Expr::Ternary(t) => visitor.visit_ternary(t),
        }
    }
}

// This trait will be used for the trait object
pub trait ExprVisitor<T> {
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
    fn visit_ternary(&self, ternary: &Ternary) -> T;
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
