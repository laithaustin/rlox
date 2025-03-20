use crate::compiler::ast::Expr;

pub struct Interpreter {
    // Interpreter state will go here
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<(), ()> {
        // Interpreter implementation will go here
        Ok(())
    }
} 