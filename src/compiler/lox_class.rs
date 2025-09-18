use crate::compiler::expr::{LoxCallable, Object};
use crate::compiler::lox_instance::LoxInstance;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    fn new(name: String) -> Self {
        LoxClass { name }
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &super::Interpreter, args: &[Object]) -> super::Result<Object> {
        let instance: LoxInstance = LoxInstance {
            klass: Box::new(self.clone()),
        };
        Ok(Object::Instance(Rc::new(instance)))
    }

    fn arity(&self) -> usize {
        return 0;
    }
}
