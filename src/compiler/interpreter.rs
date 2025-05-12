use crate::compiler::env::Env;
use crate::compiler::expr::Expr;
use crate::compiler::expr::ExprVisitor;
use crate::compiler::expr::Object;
use crate::compiler::expr::{Binary, Grouping, Literal, Ternary, Unary};
use crate::compiler::stmt::Stmt;
use crate::compiler::stmt::StmtVisitor;
use crate::compiler::token::RuntimeError;
use crate::compiler::token::TokenType;
use std::ffi::NulError;

pub struct Interpreter {
    // Interpreter state will go here
    env: std::cell::RefCell<Env>, // allows for us to mutate the environment by borrowing it mutably
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: std::cell::RefCell::new(Env::new()),
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Object, RuntimeError> {
        statement.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
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

impl StmtVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_var(&self, var: &super::stmt::Var) -> Result<Object, RuntimeError> {
        let value = var.initializer.accept(self)?;
        self.env.borrow_mut().define(var.name.lexeme.clone(), value);
        Ok(Object::Nil)
    }

    fn visit_expression(
        &self,
        expression: &super::stmt::Expression,
    ) -> Result<Object, RuntimeError> {
        Ok(expression.expression.accept(self)?)
    }

    fn visit_print(&self, print: &super::stmt::Print) -> Result<Object, RuntimeError> {
        let eval = print.expression.accept(self)?;
        println!("{:?}", eval);
        Ok(eval)
    }
}

impl ExprVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_assign(&self, assign: &super::expr::Assign) -> Result<Object, RuntimeError> {
        let value = assign.value.accept(self)?;
        Ok(self.env.borrow_mut().assign(&assign.name, value).clone())
    }

    fn visit_variable(&self, variable: &super::expr::Variable) -> Result<Object, RuntimeError> {
        Ok(self.env.borrow().get(&variable.name.lexeme).unwrap())
    }

    fn visit_literal(&self, literal: &Literal) -> Result<Object, RuntimeError> {
        match literal.value {
            Object::Number(n) => Ok(Object::Number(n)),
            Object::String(ref s) => Ok(Object::String(s.clone())),
            Object::Boolean(b) => Ok(Object::Boolean(b)),
            Object::Nil => Ok(Object::Nil),
            // Use a dummy token since Literal has no operator
            _ => {
                use crate::compiler::token::{Token, TokenType};
                let dummy_token = Token::new(TokenType::EOF, "<unknown>".to_string(), 0, None);
                Err(RuntimeError::new(dummy_token, "Unknown literal type"))
            }
        }
    }

    fn visit_unary(&self, unary: &Unary) -> Result<Object, RuntimeError> {
        let right = unary.right.accept(self)?;

        match unary.operator.token_type {
            TokenType::MINUS => {
                if let Object::Number(n) = right {
                    Ok(Object::Number(-n))
                } else {
                    Err(RuntimeError::new(
                        unary.operator.clone(),
                        "Unary minus can only be applied to numbers",
                    ))
                }
            }
            TokenType::BANG => Ok(Object::Boolean(!Interpreter::is_truthy(right))),
            _ => Err(RuntimeError::new(
                unary.operator.clone(),
                &format!("Unknown unary operator: {:?}", unary.operator.token_type),
            )),
        }
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Result<Object, RuntimeError> {
        grouping.expression.accept(self)
    }

    fn visit_binary(&self, binary: &Binary) -> Result<Object, RuntimeError> {
        let left = binary.left.accept(self)?;
        let right = binary.right.accept(self)?;

        match binary.operator.token_type {
            // basic arithmetic ops
            TokenType::MINUS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Number(l - r))
                } else {
                    Err(RuntimeError::new(
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
                _ => Err(RuntimeError::new(
                    binary.operator.clone(),
                    "Binary plus can only be applied to numbers or strings",
                )),
            },
            TokenType::SLASH => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    if r != 0.0 {
                        Ok(Object::Number(l / r))
                    } else {
                        Err(RuntimeError::new(
                            binary.operator.clone(),
                            "Division by zero",
                        ))
                    }
                } else {
                    Err(RuntimeError::new(
                        binary.operator.clone(),
                        "Binary slash can only be applied to numbers",
                    ))
                }
            }

            TokenType::STAR => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Number(l * r))
                } else {
                    Err(RuntimeError::new(
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
                    Err(RuntimeError::new(
                        binary.operator.clone(),
                        "Binary greater can only be applied to numbers",
                    ))
                }
            }

            TokenType::GREATER_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l >= r))
                } else {
                    Err(RuntimeError::new(
                        binary.operator.clone(),
                        "Binary greater equal can only be applied to numbers",
                    ))
                }
            }

            TokenType::LESS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l < r))
                } else {
                    Err(RuntimeError::new(
                        binary.operator.clone(),
                        "Binary less can only be applied to numbers",
                    ))
                }
            }

            TokenType::LESS_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Ok(Object::Boolean(l <= r))
                } else {
                    Err(RuntimeError::new(
                        binary.operator.clone(),
                        "Binary less equal can only be applied to numbers",
                    ))
                }
            }

            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),

            TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),

            _ => Err(RuntimeError::new(
                binary.operator.clone(),
                &format!("Unknown binary operator: {:?}", binary.operator.token_type),
            )),
        }
    }

    fn visit_ternary(&self, _ternary: &Ternary) -> Result<Object, RuntimeError> {
        let condition = _ternary.condition.accept(self)?;
        if Interpreter::is_truthy(condition) {
            _ternary.true_branch.accept(self)
        } else {
            _ternary.false_branch.accept(self)
        }
    }
}
