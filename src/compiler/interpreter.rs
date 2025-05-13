use crate::compiler::env::Env;
use crate::compiler::env::EnvRef;
use crate::compiler::error::{LoxError, Result};
use crate::compiler::expr::ExprVisitor;
use crate::compiler::expr::Object;
use crate::compiler::expr::{Binary, Grouping, Literal, Ternary, Unary};
use crate::compiler::stmt::Stmt;
use crate::compiler::stmt::StmtVisitor;
use crate::compiler::token::TokenType;
use std::cell::RefCell;

pub struct Interpreter {
    // Interpreter state will go here
    env: RefCell<EnvRef>, // allows for us to mutate the environment by borrowing it mutably
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: RefCell::new(Env::new_global()),
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Object> {
        statement.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn is_truthy(object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => b,
            _ => true,
        }
    }
}

impl StmtVisitor<Result<Object>> for Interpreter {
    fn visit_block(&self, block: &super::stmt::Block) -> Result<Object> {
        // update env
        let prev = self.env.borrow().clone(); // save current env
        let new_env = Env::new_enclosed(prev.clone());
        self.env.replace(new_env);

        for statement in &block.statements {
            statement.accept(self)?;
        }

        self.env.replace(prev);
        Ok(Object::Nil)
    }

    fn visit_var(&self, var: &super::stmt::Var) -> Result<Object> {
        let value = var.initializer.accept(self)?;
        self.env
            .borrow() //immutable borrow for reading the environment
            .borrow_mut() //mutable borrow for the environment inside pointer
            .define(var.name.lexeme.clone(), value);
        Ok(Object::Nil)
    }

    fn visit_expression(&self, expression: &super::stmt::Expression) -> Result<Object> {
        Ok(expression.expression.accept(self)?)
    }

    fn visit_print(&self, print: &super::stmt::Print) -> Result<Object> {
        let eval = print.expression.accept(self)?;
        println!("{:?}", eval);
        Ok(eval)
    }
}

impl ExprVisitor<Result<Object>> for Interpreter {
    fn visit_assign(&self, assign: &super::expr::Assign) -> Result<Object> {
        let value = assign.value.accept(self)?;
        let cloned_value = value.clone();
        self.env.borrow().borrow_mut().assign(&assign.name, value)?;
        Ok(cloned_value)
    }

    fn visit_variable(&self, variable: &super::expr::Variable) -> Result<Object> {
        match self
            .env
            .borrow()
            .borrow()
            .get(&variable.name.lexeme, &variable.name)
        {
            Ok(obj) => Ok(obj),
            Err(e) => Err(LoxError::new_runtime(
                variable.name.clone(),
                &format!("Undefined variable '{}'.", variable.name.lexeme),
            )),
        }
    }

    fn visit_literal(&self, literal: &Literal) -> Result<Object> {
        match literal.value {
            Object::Number(n) => Ok(Object::Number(n)),
            Object::String(ref s) => Ok(Object::String(s.clone())),
            Object::Boolean(b) => Ok(Object::Boolean(b)),
            Object::Nil => Ok(Object::Nil),
            // Use a dummy token since Literal has no operator
            Object::Error(ref msg) => {
                use crate::compiler::token::{Token, TokenType};
                let dummy_token = Token::new(TokenType::EOF, "<unknown>".to_string(), 0, None);
                Err(LoxError::new_runtime(dummy_token, msg))
            }
        }
    }

    fn visit_unary(&self, unary: &Unary) -> Result<Object> {
        let right = unary.right.accept(self)?;

        match unary.operator.token_type {
            TokenType::MINUS => {
                if let Object::Number(n) = right {
                    Ok(Object::Number(-n))
                } else {
                    Err(LoxError::new_runtime(
                        unary.operator.clone(),
                        "Unary minus can only be applied to numbers",
                    ))
                }
            }
            TokenType::BANG => Ok(Object::Boolean(!Interpreter::is_truthy(right))),
            _ => Err(LoxError::new_runtime(
                unary.operator.clone(),
                &format!("Unknown unary operator: {:?}", unary.operator.token_type),
            )),
        }
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Result<Object> {
        grouping.expression.accept(self)
    }

    fn visit_binary(&self, binary: &Binary) -> Result<Object> {
        let left = binary.left.accept(self)?;
        let right = binary.right.accept(self)?;

        match binary.operator.token_type {
            // basic arithmetic ops
            TokenType::MINUS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Number(l - r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary minus can only be applied to numbers",
                    ))
                }
            }
            TokenType::PLUS => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(*l + *r)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l.clone() + r)),
                (Object::String(l), Object::Number(r)) => {
                    Ok(Object::String(l.clone() + &r.to_string()))
                }
                (Object::Number(l), Object::String(r)) => Ok(Object::String(l.to_string() + r)),
                _ => Err(LoxError::new_runtime(
                    binary.operator.clone(),
                    "Binary plus can only be applied to numbers or strings",
                )),
            },
            TokenType::SLASH => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    if r != 0.0 {
                        Ok(Object::Number(l / r))
                    } else {
                        Err(LoxError::new_runtime(
                            binary.operator.clone(),
                            "Division by zero",
                        ))
                    }
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary slash can only be applied to numbers",
                    ))
                }
            }

            TokenType::STAR => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Number(l * r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary star can only be applied to numbers",
                    ))
                }
            }

            // comparison ops
            //
            TokenType::GREATER => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l > r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary greater can only be applied to numbers",
                    ))
                }
            }

            TokenType::GREATER_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l >= r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary greater equal can only be applied to numbers",
                    ))
                }
            }

            TokenType::LESS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l < r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary less can only be applied to numbers",
                    ))
                }
            }

            TokenType::LESS_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l <= r))
                } else {
                    Err(LoxError::new_runtime(
                        binary.operator.clone(),
                        "Binary less equal can only be applied to numbers",
                    ))
                }
            }

            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),

            TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),

            _ => Err(LoxError::new_runtime(
                binary.operator.clone(),
                &format!("Unknown binary operator: {:?}", binary.operator.token_type),
            )),
        }
    }

    fn visit_ternary(&self, _ternary: &Ternary) -> Result<Object> {
        let condition = _ternary.condition.accept(self)?;
        if Interpreter::is_truthy(condition) {
            _ternary.true_branch.accept(self)
        } else {
            _ternary.false_branch.accept(self)
        }
    }
}
