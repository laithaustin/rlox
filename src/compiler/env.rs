use crate::compiler::error::{LoxError, Result};
use crate::compiler::expr::Object;
use crate::compiler::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type EnvRef = Rc<RefCell<Env>>;

pub struct Env {
    enclosing: Option<EnvRef>,
    bindings: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            enclosing: None,
            bindings: HashMap::new(),
        }
    }

    pub fn new_global() -> EnvRef {
        Rc::new(RefCell::new(Env {
            enclosing: None,
            bindings: HashMap::new(),
        }))
    }

    pub fn new_enclosed(enclosing: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Env {
            enclosing: Some(enclosing),
            bindings: HashMap::new(),
        }))
    }

    pub fn enclose(&mut self, enclosing: Option<EnvRef>) {
        self.enclosing = enclosing;
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<&Object> {
        // key difference to define is that we error out if bindings don't exist
        if !self.bindings.contains_key(&name.lexeme) {
            return Err(LoxError::new_runtime(
                name.clone(),
                &format!("Undefined variable '{}'.", name.lexeme),
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
            None => {
                // check in enclosed first and then error out
                match &self.enclosing {
                    Some(enclosed) => enclosed.borrow().get(name, token),
                    None => Err(LoxError::new_runtime(
                        token.clone(),
                        &format!("Undefined variable '{}'.", name),
                    )),
                }
            }
        }
    }
}
