// TODO: make this pretty printing visitor work
// we can use the astvisitor to implement printing functionality
/* pub trait ExprVisitor<T> {
fn visit_binary(&self, binary: &Binary) -> T;
fn visit_grouping(&self, grouping: &Grouping) -> T;
fn visit_literal(&self, literal: &Literal) -> T;
fn visit_unary(&self, unary: &Unary) -> T;
} */

use crate::compiler::expr::ExprVisitor;
use crate::compiler::expr::{Binary, Grouping, Literal, Unary};

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &Binary) -> String {
        format!(
            "({:?} {:?} {:?})",
            binary.operator.lexeme,
            binary.left.accept(self),
            binary.right.accept(self)
        )
    }

    fn visit_grouping(&self, grouping: &Grouping) -> String {
        format!("(group {:?})", grouping.expression.accept(self))
    }

    fn visit_literal(&self, literal: &Literal) -> String {
        format!("{:?}", literal.value)
    }

    fn visit_unary(&self, unary: &Unary) -> String {
        format!(
            "({:?} {:?})",
            unary.operator.lexeme,
            unary.right.accept(self)
        )
    }
}
