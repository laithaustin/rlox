use crate::compiler::token::Token;
use crate::compiler::expr::Object;

#[derive(Debug, Clone)]
pub enum stmt {
    Expression(Box<Expression>),
    Print(Box<Print>),
}

pub trait stmtVisitor<T> {
    fn visit_expression(&self, expression: &Expression) -> T;
    fn visit_print(&self, print: &Print) -> T;
}

impl stmt {
    pub fn accept<T>(&self, visitor: &dyn stmtVisitor<T>) -> T {
        match self {
            stmt::Expression(b) => visitor.visit_expression(b),
            stmt::Print(b) => visitor.visit_print(b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<Expr>,
}

