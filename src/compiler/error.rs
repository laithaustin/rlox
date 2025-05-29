use crate::compiler::token::{Token, TokenType};
use std::fmt;

// Define different error types in our interpreter
#[derive(Debug, Clone)]
pub enum LoxErrorKind {
    // Syntax errors during parsing
    Parse,
    // Runtime errors during execution
    Runtime,
    // General interpreter errors
    Internal,
}

#[derive(Debug, Clone)]
pub struct LoxError {
    pub kind: LoxErrorKind,
    pub message: String,
    pub token: Option<Token>,
    pub line: Option<usize>,
}

impl LoxError {
    pub fn new_runtime(token: Token, message: &str) -> Self {
        LoxError {
            kind: LoxErrorKind::Runtime,
            message: message.to_string(),
            token: Some(token),
            line: None,
        }
    }

    pub fn new_parse(token: Token, message: &str) -> Self {
        LoxError {
            kind: LoxErrorKind::Parse,
            message: message.to_string(),
            token: Some(token),
            line: None,
        }
    }

    pub fn new_from_line(line: usize, message: &str) -> Self {
        LoxError {
            kind: LoxErrorKind::Parse,
            message: message.to_string(),
            token: None,
            line: Some(line),
        }
    }

    pub fn new_internal(message: &str) -> Self {
        LoxError {
            kind: LoxErrorKind::Internal,
            message: message.to_string(),
            token: None,
            line: None,
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LoxErrorKind::Runtime => {
                if let Some(token) = &self.token {
                    write!(
                        f,
                        "[line {}] Runtime Error at '{}': {}",
                        token.line, token.lexeme, self.message
                    )
                } else {
                    write!(f, "Runtime Error: {}", self.message)
                }
            }
            LoxErrorKind::Parse => {
                if let Some(token) = &self.token {
                    if token.token_type == TokenType::EOF {
                        write!(f, "[line {}] Error at end: {}", token.line, self.message)
                    } else {
                        write!(
                            f,
                            "[line {}] Error at '{}': {}",
                            token.line, token.lexeme, self.message
                        )
                    }
                } else if let Some(line) = self.line {
                    write!(f, "[line {}] Error: {}", line, self.message)
                } else {
                    write!(f, "Parse Error: {}", self.message)
                }
            }
            LoxErrorKind::Internal => {
                write!(f, "Internal Error: {}", self.message)
            }
        }
    }
}

// Trait for reporting errors
pub trait ErrorReporter {
    fn error(&mut self, line: usize, message: &str);
    fn runtime_error(&mut self, error: &LoxError);
}

// Define a type alias for our result type that uses LoxError
pub type Result<T> = std::result::Result<T, LoxError>;
