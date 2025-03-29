use crate::compiler::token::Token;

// Define Object type to represent Lox values
#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

// This trait will be used for the trait object
pub trait Expr {
    fn accept_visitor<T>(&self, visitor: &dyn ExprVisitor<T>) -> T;
}

pub trait ExprVisitor<T> {
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_literal(&self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Unary {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_unary(self)
    }
}

impl Expr for Binary {
    fn accept_visitor<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        self.accept(visitor)
    }
}

impl Expr for Grouping {
    fn accept_visitor<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        self.accept(visitor)
    }
}

impl Expr for Literal {
    fn accept_visitor<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        self.accept(visitor)
    }
}

impl Expr for Unary {
    fn accept_visitor<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        self.accept(visitor)
    }
}
