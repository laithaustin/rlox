use crate::compiler::Interpreter;
use crate::compiler::env::{Env, EnvGuard, EnvRef};
use crate::compiler::expr::{LoxCallable, Object};
use crate::compiler::stmt::Function;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: Function,
    pub closure: EnvRef,
}

impl LoxFunction {
    fn new(declaration: Function, closure: EnvRef) -> Self {
        LoxFunction {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &super::Interpreter, args: &[Object]) -> super::Result<Object> {
        // need to create a new env and bind the variables to it
        let env = Env::new_enclosed(self.closure.clone());

        for (i, param) in self.declaration.parameters.iter().enumerate() {
            env.borrow_mut()
                .define(param.lexeme.clone(), args[i].clone());
        }

        // execute body
        let _guard = EnvGuard::new(interpreter, env);
        
        // The function body is a Block statement, so we need to extract its statements
        match self.declaration.body.as_ref() {
            crate::compiler::stmt::Stmt::Block(block) => {
                for stmt in &block.statements {
                    stmt.accept(interpreter)?;
                }
            }
            _ => {
                // If it's not a block, just execute the single statement
                self.declaration.body.accept(interpreter)?;
            }
        }

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
