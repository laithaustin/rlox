use crate::compiler::expr::Expr;
use crate::compiler::expr::Object;
use crate::compiler::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Box<Expression>),
    Print(Box<Print>),
    Var(Box<Var>),
    Block(Box<Block>),
    IfStmt(Box<IfStmt>),
    WhileStmt(Box<WhileStmt>),
}

pub trait StmtVisitor<T> {
    fn visit_expression(&self, expression: &Expression) -> T;
    fn visit_print(&self, print: &Print) -> T;
    fn visit_var(&self, var: &Var) -> T;
    fn visit_block(&self, block: &Block) -> T;
    fn visit_if_stmt(&self, if_stmt: &IfStmt) -> T;
    fn visit_while_stmt(&self, while_stmt: &WhileStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(b) => visitor.visit_expression(b),
            Stmt::Print(b) => visitor.visit_print(b),
            Stmt::Var(b) => visitor.visit_var(b),
            Stmt::Block(b) => visitor.visit_block(b),
            Stmt::IfStmt(b) => visitor.visit_if_stmt(b),
            Stmt::WhileStmt(b) => visitor.visit_while_stmt(b),
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

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}
