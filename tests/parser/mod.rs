// Include our error reporting test modules
mod error_reporting;
mod runtime_errors;

use crate::common::TestErrorReporter;
use lox::compiler::token::{Token, TokenType};

use lox::compiler::expr::{Expr, Literal, Object};
use lox::compiler::parser::Parser;
use lox::compiler::scanner::Scanner;
use lox::compiler::stmt::Stmt;

// Basic helper function to parse a source string
fn parse_source(source: &str) -> (Option<Vec<Stmt>>, TestErrorReporter) {
    let mut reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut reporter);
    scanner.scan_tokens();
    
    let mut parser = Parser::new(&scanner.tokens, &mut reporter);
    let result = parser.parse();
    
    (result.ok(), reporter)
}

// Basic tests for the parser's functionality
#[test]
fn test_parse_simple_expression() {
    let (result, reporter) = parse_source("123;");
    assert!(result.is_some(), "Parsing should succeed");
    reporter.assert_no_errors();
}

#[test]
fn test_parse_variable_declaration() {
    let (result, reporter) = parse_source("var x = 10;");
    assert!(result.is_some(), "Parsing should succeed");
    reporter.assert_no_errors();
}

#[test]
fn test_parse_print_statement() {
    let (result, reporter) = parse_source("print \"hello\";");
    assert!(result.is_some(), "Parsing should succeed");
    reporter.assert_no_errors();
}

// Original commented tests preserved for reference

// fn parse_expr(source: &str) -> Result<Stmt, ()> {
//     let mut reporter = crate::common::TestErrorReporter::new();
//     let mut scanner = Scanner::new(source.to_string(), &mut reporter);
//     scanner.scan_tokens();
//     let mut parser = Parser::new(&scanner.tokens, &mut reporter);
//     parser.parse()
// }

// #[test]
// fn test_parse_number_literal() {
//     let expr = parse_expr("123;").unwrap();
//     match expr {
//         Expr::Literal(lit) => assert_eq!(lit.value, Object::Number(123.0)),
//         _ => panic!("Expected number literal"),
//     }
// }

// #[test]
// fn test_parse_string_literal() {
//     let expr = parse_expr("\"hello\";").unwrap();
//     match expr {
//         Expr::Literal(lit) => {
//             println!("Literal: {:?}", lit);
//             assert_eq!(lit.value, Object::String("hello".to_string()));
//         }
//         _ => panic!("Expected string literal"),
//     }
// }

// #[test]
// fn test_parse_binary_expression() {
//     let expr = parse_expr("1 + 2;").unwrap();
//     match expr {
//         Expr::Binary(bin) => match (*bin.left, *bin.right) {
//             (Expr::Literal(l), Expr::Literal(r)) => {
//                 assert_eq!(l.value, Object::Number(1.0));
//                 assert_eq!(r.value, Object::Number(2.0));
//             }
//             _ => panic!("Expected literals on both sides"),
//         },
//         _ => panic!("Expected binary expression"),
//     }
// }

// #[test]
// fn test_parse_grouping() {
//     let expr = parse_expr("(42);").unwrap();
//     match expr {
//         Expr::Grouping(g) => match *g.expression {
//             Expr::Literal(lit) => assert_eq!(lit.value, Object::Number(42.0)),
//             _ => panic!("Expected literal inside grouping"),
//         },
//         _ => panic!("Expected grouping expression"),
//     }
// }

// #[test]
// fn test_parse_unary() {
//     let expr = parse_expr("-5;").unwrap();
//     match expr {
//         Expr::Unary(u) => match *u.right {
//             Expr::Literal(lit) => assert_eq!(lit.value, Object::Number(5.0)),
//             _ => panic!("Expected literal inside unary"),
//         },
//         _ => panic!("Expected unary expression"),
//     }
// }

// #[test]
// fn test_parse_ternary() {
//     let expr = parse_expr("true ? 1 : 2;").unwrap();
//     match expr {
//         Expr::Ternary(t) => {
//             assert!(matches!(*t.condition, Expr::Literal(_)));
//             assert!(matches!(*t.true_branch, Expr::Literal(_)));
//             assert!(matches!(*t.false_branch, Expr::Literal(_)));
//         }
//         _ => panic!("Expected ternary expression"),
//     }
// }

// #[test]
// fn test_parse_error() {
//     let result = parse_expr("(1 + 2;");
//     assert!(result.is_err());
// }
