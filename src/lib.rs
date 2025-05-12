pub mod compiler;

// Re-export important types for easier use
pub use compiler::error::{LoxError, LoxErrorKind, Result, ErrorReporter};
pub use compiler::interpreter::Interpreter;
pub use compiler::parser::Parser;
pub use compiler::scanner::Scanner;
pub use compiler::expr::{Expr, Object};
pub use compiler::stmt::Stmt;