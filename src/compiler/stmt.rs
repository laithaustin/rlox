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
    Function(Box<Function>),
    Class(Box<Class>),
    ReturnStmt(Box<ReturnStmt>),
}

pub trait StmtVisitor<T> {
    fn visit_expression(&self, expression: &Expression) -> T;
    fn visit_print(&self, print: &Print) -> T;
    fn visit_var(&self, var: &Var) -> T;
    fn visit_block(&self, block: &Block) -> T;
    fn visit_if_stmt(&self, if_stmt: &IfStmt) -> T;
    fn visit_while_stmt(&self, while_stmt: &WhileStmt) -> T;
    fn visit_function(&self, function: &Function) -> T;
    fn visit_class(&self, class: &Class) -> T;
    fn visit_return_stmt(&self, return_stmt: &ReturnStmt) -> T;
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
            Stmt::Function(b) => visitor.visit_function(b),
            Stmt::Class(b) => visitor.visit_class(b),
            Stmt::ReturnStmt(b) => visitor.visit_return_stmt(b),
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

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Box<Token>,
    pub parameters: Box<Vec<Token>>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Box<Token>,
    pub methods: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub tok: Box<Token>,
    pub value: Box<Expr>,
}
