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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    NONE,
    FUNCTION,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarState {
    DECL,
    DEF,
    USE,
}

pub struct Resolver {
    pub interpreter: Rc<RefCell<Interpreter>>,
    pub scopes: RefCell<Vec<HashMap<String, VarState>>>,
    pub errors: RefCell<Vec<LoxError>>,
    pub current_function: RefCell<FunctionType>,
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
        // For self-reference detection, we need to declare the variable
        // before resolving its initializer, even in global scope
        let was_global = self.scopes.borrow().is_empty();

        if was_global {
            // Create temporary scope for global variable self-reference detection
            self.begin_scope();
        }

        self.declare(&var.name);
        self.resolve_expression(&var.initializer);
        self.define(&var.name);

        if was_global {
            // Remove temporary scope
            self.end_scope();
        }
    }

    fn visit_function(&self, function: &super::stmt::Function) -> () {
        // Handle function declaration similar to variables but allow immediate use
        let was_global = self.scopes.borrow().is_empty();

        if was_global {
            // For global functions, we don't need scope management
            // They should be available in global environment
        } else {
            // For local functions, declare and define immediately
            // (functions are hoisted and can be called before definition)
            self.declare(&function.name);
            self.define(&function.name);
        }

        self.resolve_function(function);
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
        if *self.current_function.borrow() == FunctionType::NONE {
            self.errors.borrow_mut().push(LoxError::new_parse(
                return_stmt.tok.as_ref().clone(),
                "Cannot return from top level code.",
            ))
        }
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
            // Check for self-referential initialization in current scope only
            if let Some(VarState::DECL) = self
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

            // Mark the variable as used in whichever scope it's defined in
            let mut scopes = self.scopes.borrow_mut();
            for scope in scopes.iter_mut().rev() {
                if let Some(current_state) = scope.get(&variable.name.lexeme).cloned() {
                    if current_state == VarState::DEF {
                        scope.insert(variable.name.lexeme.clone(), VarState::USE);
                    }
                    break; // Found the variable, stop searching
                }
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
        // iterate through current scope and check for variable that are decl or def
        for (name, kind) in self.scopes.borrow_mut().pop().unwrap().iter() {
            match kind {
                VarState::DECL => {
                    self.errors
                        .borrow_mut()
                        .push(LoxError::new_warning(&format!(
                            "Variable '{}' is declared but never used",
                            name
                        )));
                }
                VarState::DEF => {
                    self.errors
                        .borrow_mut()
                        .push(LoxError::new_warning(&format!(
                            "Variable '{}' is defined but never used",
                            name
                        )));
                }
                VarState::USE => {}
            }
        }
    }

    pub fn resolve_function(&self, func: &Function) {
        // stash our function status - need to traxk when we enter and exit
        let enclosing_function: FunctionType = self.current_function.borrow().clone();
        self.current_function.replace(FunctionType::FUNCTION);

        self.begin_scope();
        for param in func.parameters.iter() {
            self.declare(param);
            self.define(param);
        }

        // resolve function body - since it's always a Block, resolve its statements directly
        // instead of creating another scope
        match func.body.as_ref() {
            crate::compiler::stmt::Stmt::Block(block) => {
                self.resolve_statements(&block.statements);
            }
            _ => {
                // If it's not a block (shouldn't happen), resolve it normally
                self.resolve_statement(func.body.as_ref());
            }
        }
        self.end_scope();
        self.current_function.replace(enclosing_function);
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
            self.errors.borrow_mut().push(LoxError::new_internal(
                "Already a variable with this name in this scope",
            ));
        }
        current.insert(var.lexeme.clone(), VarState::DECL);
    }

    pub fn define(&self, var: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }

        if let Some(scope) = self.scopes.borrow_mut().last_mut() {
            // Only update to DEF if not already USE
            if let Some(&current_state) = scope.get(&var.lexeme) {
                if current_state != VarState::USE {
                    scope.insert(var.lexeme.clone(), VarState::DEF);
                }
            } else {
                scope.insert(var.lexeme.clone(), VarState::DEF);
            }
        }
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
            current_function: RefCell::new(FunctionType::NONE),
        }
    }
}
