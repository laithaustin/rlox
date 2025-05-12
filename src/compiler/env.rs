use crate::compiler::expr::Object;
use crate::compiler::error::{LoxError, Result};
use crate::compiler::token::Token;
use std::collections::HashMap;

pub struct Env {
    bindings: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            bindings: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<&Object> {
        // key difference to define is that we error out if bindings don't exist
        if !self.bindings.contains_key(&name.lexeme) {
            return Err(LoxError::new_runtime(
                name.clone(),
                &format!("Undefined variable '{}'.", name.lexeme)
            ));
        }
        self.bindings.insert(name.lexeme.clone(), value);
        Ok(&self.bindings[&name.lexeme])
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: &String, token: &Token) -> Result<Object> {
        match self.bindings.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxError::new_runtime(
                token.clone(),
                &format!("Undefined variable '{}'.", name)
            ))
        }
    }
}
