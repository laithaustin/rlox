// Abstract Syntax Tree implementation
// This will contain the AST nodes and related functionality

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary {
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Ast {
    // AST implementation will go here
} 