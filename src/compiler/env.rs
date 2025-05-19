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
        // Check if the variable exists in the current environment
        if self.bindings.contains_key(&name.lexeme) {
            self.bindings.insert(name.lexeme.clone(), value);
            return Ok(&self.bindings[&name.lexeme]);
        }
        
        // If not in current environment, check in enclosing environments
        match &self.enclosing {
            Some(enclosed) => {
                enclosed.borrow_mut().assign(name, value)?;
                // Return a reference to the value in this environment 
                // (this is slightly inconsistent since the value is actually in the parent)
                Ok(&self.bindings.get(&name.lexeme).unwrap_or(&Object::Nil))
            }
            None => Err(LoxError::new_runtime(
                name.clone(),
                &format!("Undefined variable '{}' during assign.", name.lexeme),
            )),
        }
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
                        &format!("Undefined variable '{}' during get.", name),
                    )),
                }
            }
        }
    }
}
