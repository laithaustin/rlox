use lox::compiler::{Scanner, Token};
use lox::compiler::token::TokenType;
use crate::common::TestErrorReporter;
use test_case::test_case;

mod basic;
mod literals;
mod keywords;
mod errors;
mod comments;
mod edge_cases;

// Helper function to create a scanner and get tokens
fn scan(input: &str) -> (Vec<Token>, TestErrorReporter) {
    let mut reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(input.to_string(), &mut reporter);
    scanner.scan_tokens();
    (scanner.tokens, reporter)
}

// Helper function to assert token properties
fn assert_token(token: &Token, expected_type: TokenType, expected_lexeme: &str, expected_line: usize) {
    assert_eq!(token.token_type, expected_type);
    assert_eq!(token.lexeme, expected_lexeme);
    assert_eq!(token.line, expected_line);
}

// Helper function to assert token sequence
fn assert_token_sequence(tokens: &[Token], expected_types: &[TokenType]) {
    assert_eq!(tokens.len() - 1, expected_types.len(), "Token count mismatch (excluding EOF)");
    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
    assert_eq!(tokens.last().unwrap().token_type, TokenType::EOF);
}

// Re-export test modules
pub use basic::*;
pub use literals::*;
pub use keywords::*;
pub use errors::*;
pub use comments::*;
pub use edge_cases::*; 