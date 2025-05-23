use crate::compiler::error::{LoxError, Result};
use crate::compiler::expr::{LoxCallable, Object};
use crate::compiler::interpreter::Interpreter;
use crate::compiler::token::{Token, TokenType};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ClockFunction;

impl LoxCallable for ClockFunction {
    fn call(&self, _interpreter: &Interpreter, _args: &[Object]) -> Result<Object> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| {
                LoxError::new_runtime(
                    Token::new(TokenType::EOF, "clock".to_string(), 0, None),
                    "System time error",
                )
            })?
            .as_secs_f64();
        Ok(Object::Number(now))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "<native fn clock>".to_string()
    }
}
