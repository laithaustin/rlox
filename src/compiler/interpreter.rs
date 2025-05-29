use crate::compiler::env::{Env, EnvGuard, EnvRef};
use crate::compiler::error::{LoxError, Result};
use crate::compiler::expr::ExprVisitor;
use crate::compiler::expr::Object;
use crate::compiler::expr::{Binary, Grouping, Literal, LoxCallable, Ternary, Unary};
use crate::compiler::lox_function::LoxFunction;
use crate::compiler::natives::ClockFunction;
use crate::compiler::stmt::Stmt;
use crate::compiler::stmt::StmtVisitor;
use crate::compiler::token::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    // Interpreter state will go here
    pub _globals: EnvRef,
    pub env: RefCell<EnvRef>, // allows for us to mutate the environment by borrowing it mutably
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Env::new_global();

        // add native functions here
        // let's add one for counting time
        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Function(Rc::new(ClockFunction)),
        );

        Interpreter {
            _globals: globals.clone(),
            env: RefCell::new(globals),
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

    pub fn execute_block(&self, statements: &Vec<Stmt>, new_env: EnvRef) -> Result<()> {
        let _guard = EnvGuard::new(self, new_env);
        for statement in statements.iter() {
            statement.accept(self)?;
        }
        Ok(())
    }
}

impl StmtVisitor<Result<Object>> for Interpreter {
    fn visit_function(&self, function: &super::stmt::Function) -> Result<Object> {
        // first create function object using current env and ast node
        let lox_function = LoxFunction {
            declaration: function.clone(),
            closure: self.env.borrow().clone(),
        };
        // make sure to create a new shared reference to the function object
        let function_obj = Object::Function(Rc::new(lox_function));

        // inject into the environment
        self.env
            .borrow()
            .borrow_mut()
            .define(function.name.lexeme.clone(), function_obj);

        Ok(Object::Nil)
    }

    fn visit_while_stmt(&self, while_stmt: &super::stmt::WhileStmt) -> Result<Object> {
        while Interpreter::is_truthy(while_stmt.condition.accept(self)?) {
            while_stmt.body.accept(self)?;
        }
        Ok(Object::Nil)
    }

    fn visit_if_stmt(&self, if_stmt: &super::stmt::IfStmt) -> Result<Object> {
        let cond = if_stmt.condition.accept(self)?;
        if Interpreter::is_truthy(cond) {
            if_stmt.then_branch.accept(self)?;
        } else if let Some(else_branch) = &if_stmt.else_branch {
            else_branch.accept(self)?;
        }
        Ok(Object::Nil)
    }

    fn visit_block(&self, block: &super::stmt::Block) -> Result<Object> {
        // update env
        let new_env = Env::new_enclosed(self.env.borrow().clone());
        self.execute_block(&block.statements, new_env)?;
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
    fn visit_call(&self, call: &super::expr::Call) -> Result<Object> {
        let callee = call.callee.accept(self)?;
        let args = call
            .args
            .iter()
            .map(|arg| arg.accept(self))
            .collect::<Result<Vec<Object>>>()?;
        match callee {
            Object::Function(function) => {
                // Check arity first
                if args.len() != function.arity() {
                    return Err(LoxError::new_runtime(
                        call.paren.clone(),
                        &format!(
                            "Expected {} arguments but got {}.",
                            function.arity(),
                            args.len()
                        ),
                    ));
                }
                function.call(self, &args)
            }
            _ => Err(LoxError::new_runtime(
                call.paren.clone(),
                "Can only call functions.",
            )),
        }
    }

    fn visit_logical(&self, logical: &super::expr::Logical) -> Result<Object> {
        // need to short circuit after evaluating left
        let left = logical.left.accept(self)?;
        if logical.operator.token_type == TokenType::OR {
            if Interpreter::is_truthy(left.clone()) {
                Ok(left)
            } else {
                logical.right.accept(self)
            }
        } else {
            if !Interpreter::is_truthy(left.clone()) {
                Ok(left)
            } else {
                logical.right.accept(self)
            }
        }
    }

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
            Err(_) => Err(LoxError::new_runtime(
                variable.name.clone(),
                &format!(
                    "Undefined variable '{}' during visit.",
                    variable.name.lexeme
                ),
            )),
        }
    }

    fn visit_literal(&self, literal: &Literal) -> Result<Object> {
        match literal.value {
            Object::Number(n) => Ok(Object::Number(n)),
            Object::String(ref s) => Ok(Object::String(s.clone())),
            Object::Boolean(b) => Ok(Object::Boolean(b)),
            Object::Nil => Ok(Object::Nil),
            Object::Function(ref f) => Ok(Object::Function(f.clone())),
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
