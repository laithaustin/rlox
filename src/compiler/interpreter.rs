use crate::compiler::expr::Expr;
use crate::compiler::expr::ExprVisitor;
use crate::compiler::expr::Object;
use crate::compiler::expr::{Binary, Grouping, Literal, Ternary, Unary};
use crate::compiler::token::TokenType;

pub struct Interpreter {
    // Interpreter state will go here
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expr: &Expr) -> Object {
        expr.accept(self)
    }

    fn is_truthy(object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => b,
            _ => true,
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal(&self, literal: &Literal) -> Object {
        match literal.value {
            Object::Number(n) => Object::Number(n),
            Object::String(ref s) => Object::String(s.clone()),
            Object::Boolean(b) => Object::Boolean(b),
            Object::Nil => Object::Nil,
            _ => panic!("Unknown literal type"), // TODO: error handle this
        }
    }

    fn visit_unary(&self, unary: &Unary) -> Object {
        let right = unary.right.accept(self);

        match unary.operator.token_type {
            TokenType::MINUS => {
                if let Object::Number(n) = right {
                    // fun fact: if let is a pattern matching
                    // expression
                    Object::Number(-n)
                } else {
                    panic!("Unary minus can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::BANG => {
                if Interpreter::is_truthy(right) {
                    Object::Boolean(false)
                } else {
                    Object::Boolean(true)
                }
            }

            _ => {
                panic!("Unknown unary operator: {:?}", unary.operator.token_type);
            }
        }
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Object {
        return grouping.expression.accept(self);
    }

    fn visit_binary(&self, binary: &Binary) -> Object {
        let left = binary.left.accept(self);
        let right = binary.right.accept(self);

        match binary.operator.token_type {
            // basic arithmetic ops
            TokenType::MINUS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Object::Number(l - r);
                } else {
                    panic!("Binary minus can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::PLUS => {
                match (&left, &right) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(*l + *r),
                    (Object::String(l), Object::String(r)) => Object::String(l.clone() + r),
                    (Object::String(l), Object::Number(r)) => {
                        Object::String(l.clone() + &r.to_string())
                    }
                    (Object::Number(l), Object::String(r)) => Object::String(l.to_string() + r),
                    _ => panic!("Binary plus can only be applied to numbers or strings"), // TODO: error handle this
                }
            }

            TokenType::SLASH => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    if r != 0.0 {
                        Object::Number(l / r)
                    } else {
                        panic!("Division by zero")
                    }
                } else {
                    panic!("Binary slash can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::STAR => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Object::Number(l * r)
                } else {
                    panic!("Binary star can only be applied to numbers") // TODO: error handle this
                }
            }

            // comparison ops
            //
            TokenType::GREATER => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Object::Boolean(l > r)
                } else {
                    panic!("Binary greater can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::GREATER_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Object::Boolean(l >= r)
                } else {
                    panic!("Binary greater equal can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::LESS => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Object::Boolean(l < r)
                } else {
                    panic!("Binary less can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::LESS_EQUAL => {
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    Object::Boolean(l <= r)
                } else {
                    panic!("Binary less equal can only be applied to numbers") // TODO: error handle this
                }
            }

            TokenType::EQUAL_EQUAL => {
                if left == right {
                    Object::Boolean(true)
                } else {
                    Object::Boolean(false)
                }
            }

            TokenType::BANG_EQUAL => {
                if left != right {
                    Object::Boolean(true)
                } else {
                    Object::Boolean(false)
                }
            }

            _ => {
                panic!("Unknown binary operator: {:?}", binary.operator.token_type);
            }
        }
    }

    fn visit_ternary(&self, _ternary: &Ternary) -> Object {
        let condition = _ternary.condition.accept(self);
        if Interpreter::is_truthy(condition) {
            _ternary.true_branch.accept(self)
        } else {
            _ternary.false_branch.accept(self)
        }
    }
}
