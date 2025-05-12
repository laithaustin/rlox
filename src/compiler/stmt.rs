use crate::compiler::expr::Expr;
use crate::compiler::expr::Object;
use crate::compiler::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Box<Expression>),
    Print(Box<Print>),
    Var(Box<Var>),
}

pub trait StmtVisitor<T> {
    fn visit_expression(&self, expression: &Expression) -> T;
    fn visit_print(&self, print: &Print) -> T;
    fn visit_var(&self, var: &Var) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(b) => visitor.visit_expression(b),
            Stmt::Print(b) => visitor.visit_print(b),
            Stmt::Var(b) => visitor.visit_var(b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Box<Token>,
    pub initializer: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<Expr>,
}
