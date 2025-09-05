use crate::compiler::Result;
use crate::compiler::interpreter::Interpreter;
use crate::compiler::token::Token;
use std::fmt;
use std::rc::Rc;

// Define Object type to represent Lox values
#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Error(String),
    Function(Rc<dyn LoxCallable>),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Error(a), Object::Error(b)) => a == b,
            // Functions are only equal if they're the same reference
            (Object::Function(a), Object::Function(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Error(e) => write!(f, "Error: {}", e),
            Object::Function(func) => write!(f, "{}", func.to_string()),
        }
    }
}

// Enum-based AST representation
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
    Ternary(Box<Ternary>),
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Logical(Box<Logical>),
    Call(Box<Call>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(b) => visitor.visit_binary(b),
            Expr::Grouping(g) => visitor.visit_grouping(g),
            Expr::Literal(l) => visitor.visit_literal(l),
            Expr::Unary(u) => visitor.visit_unary(u),
            Expr::Ternary(t) => visitor.visit_ternary(t),
            Expr::Variable(v) => visitor.visit_variable(v),
            Expr::Assign(a) => visitor.visit_assign(a),
            Expr::Logical(l) => visitor.visit_logical(l),
            Expr::Call(c) => visitor.visit_call(c),
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
    fn visit_variable(&self, variable: &Variable) -> T;
    fn visit_assign(&self, assign: &Assign) -> T;
    fn visit_logical(&self, logical: &Logical) -> T;
    fn visit_call(&self, call: &Call) -> T;
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

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Expr>,
}

pub trait LoxCallable: std::fmt::Debug {
    fn call(&self, interpreter: &Interpreter, args: &[Object]) -> Result<Object>;
    fn arity(&self) -> usize;
    // Functions can override this to provide a string representation
    fn to_string(&self) -> String {
        "<fn>".to_string()
    }
}

// We'll implement specific callable types later when needed
