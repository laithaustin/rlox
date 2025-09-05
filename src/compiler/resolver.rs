use crate::compiler::Interpreter;
use crate::compiler::error::{LoxError, Result};
use crate::compiler::expr::Expr;
use crate::compiler::expr::ExprVisitor;
use crate::compiler::stmt::Stmt;
use crate::compiler::stmt::StmtVisitor;
use crate::compiler::token::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::expr::Variable;
use super::stmt::Function;

pub struct Resolver {
    pub interpreter: Rc<RefCell<Interpreter>>,
    pub scopes: RefCell<Vec<HashMap<String, bool>>>,
    pub errors: RefCell<Vec<LoxError>>,
}

// Our primary concerns for this semantic analysis are for the following cases:
// 1. Resolving new scopes within blocks
// 2. Variable and assignment expressions need to be resolved
// 3. Variable declarations add a new variable to scope
// 4. Function declarations add a new scope for the body and params

impl StmtVisitor<()> for Resolver {
    fn visit_block(&self, block: &super::stmt::Block) -> () {
        self.begin_scope();
        self.resolve_statements(&block.statements);
        self.end_scope();
    }

    fn visit_var(&self, var: &super::stmt::Var) -> () {
        self.declare(&var.name);
        self.resolve_expression(&var.initializer);
        self.define(&var.name);
    }

    fn visit_function(&self, function: &super::stmt::Function) -> () {
        self.declare(&function.name);
        self.define(&function.name);
        self.resolve_function(&function);
    }

    fn visit_expression(&self, expression: &super::stmt::Expression) -> () {
        self.resolve_expression(&expression.expression);
    }

    fn visit_if_stmt(&self, if_stmt: &super::stmt::IfStmt) -> () {
        self.resolve_expression(&if_stmt.condition);
        self.resolve_statement(&if_stmt.then_branch);
        if let Some(else_branch) = &if_stmt.else_branch {
            self.resolve_statement(else_branch);
        }
    }

    fn visit_print(&self, print: &super::stmt::Print) -> () {
        self.resolve_expression(&print.expression);
    }

    fn visit_return_stmt(&self, return_stmt: &super::stmt::ReturnStmt) -> () {
        self.resolve_expression(&return_stmt.value);
    }

    fn visit_while_stmt(&self, while_stmt: &super::stmt::WhileStmt) -> () {
        self.resolve_expression(&while_stmt.condition);
        self.resolve_statement(&while_stmt.body);
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_variable(&self, variable: &Variable) -> () {
        if !self.scopes.borrow().is_empty() {
            if let Some(false) = self
                .scopes
                .borrow()
                .last()
                .unwrap()
                .get(&variable.name.lexeme)
            {
                self.error(
                    &variable.name,
                    "Can't read local variable in its own initializer.",
                );
            }
        }
        self.resolve_local(&variable.name);
    }

    fn visit_call(&self, call: &super::expr::Call) -> () {
        self.resolve_expression(&call.callee);
        for arg in &call.args {
            self.resolve_expression(arg);
        }
    }

    fn visit_binary(&self, binary: &super::expr::Binary) -> () {
        self.resolve_expression(&binary.left);
        self.resolve_expression(&binary.right);
    }

    fn visit_assign(&self, assign: &super::expr::Assign) -> () {
        self.resolve_expression(&assign.value);
        self.resolve_local(&assign.name);
    }

    fn visit_logical(&self, logical: &super::expr::Logical) -> () {
        self.resolve_expression(&logical.left);
        self.resolve_expression(&logical.right);
    }

    fn visit_unary(&self, unary: &super::expr::Unary) -> () {
        self.resolve_expression(&unary.right);
    }

    fn visit_ternary(&self, ternary: &super::expr::Ternary) -> () {
        self.resolve_expression(&ternary.condition);
        self.resolve_expression(&ternary.false_branch);
        self.resolve_expression(&ternary.true_branch);
    }

    fn visit_grouping(&self, grouping: &super::expr::Grouping) -> () {
        self.resolve_expression(&grouping.expression);
    }

    fn visit_literal(&self, literal: &super::expr::Literal) -> () {}
}

impl Resolver {
    pub fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    pub fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    pub fn resolve_function(&self, func: &Function) {
        self.begin_scope();
        for param in func.parameters.iter() {
            self.declare(param);
            self.define(param);
        }

        // resolve function body
        match func.body.as_ref() {
            Stmt::Block(block) => {
                self.resolve_statements(&block.statements);
            }
            _ => {
                self.resolve_statement(func.body.as_ref());
            }
        }

        self.end_scope();
    }

    pub fn resolve_local(&self, name: &Token) {
        // iterate backwards through scopes to find appropriate variable to resolve
        for (i, scope) in self.scopes.borrow().iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .borrow()
                    .resolve(name, self.scopes.borrow().len() - i - 1);
                return;
            }
        }
    }

    pub fn declare(&self, var: &Token) {
        {
            let scopes = self.scopes.borrow();
            if scopes.is_empty() {
                return;
            }
        }

        let mut scopes = self.scopes.borrow_mut();
        let current = scopes.last_mut().unwrap();
        if current.contains_key(&var.lexeme) {
            eprintln!("Variable '{}' already declared in this scope.", var.lexeme);
        }
        current.insert(var.lexeme.clone(), false);
    }

    pub fn define(&self, var: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }

        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(var.lexeme.clone(), true);
    }

    pub fn resolve_statements(&self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    pub fn resolve_statement(&self, statement: &Stmt) {
        statement.accept(self);
    }

    pub fn resolve_expression(&self, expr: &Expr) {
        expr.accept(self);
    }

    pub fn error(&self, token: &Token, err_msg: &str) {
        self.errors
            .borrow_mut()
            .push(LoxError::new_parse(token.clone(), err_msg));
    }

    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()), // aggregate errors as we go
        }
    }
}
