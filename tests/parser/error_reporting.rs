use crate::common::TestErrorReporter;
use lox::compiler::parser::Parser;
use lox::compiler::scanner::Scanner;
use lox::compiler::stmt::Stmt;
use lox::compiler::error::ErrorReporter;

// Helper function to parse a source string and return statements or error
fn parse(source: &str) -> (Option<Vec<Stmt>>, TestErrorReporter) {
    let mut reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut reporter);
    scanner.scan_tokens();
    
    let mut parser = Parser::new(&scanner.tokens, &mut reporter);
    let result = parser.parse();
    
    if let Err(err) = &result {
        // When we have an error from the parser itself, make sure it's reported
        // This is needed because our new error mechanism returns errors directly
        // instead of just calling the error reporter
        reporter.error(1, &err.to_string());
    }
    
    (result.ok(), reporter)
}

#[test]
fn test_missing_semicolon() {
    let (result, reporter) = parse("var x = 10");
    assert!(result.is_none(), "Expected parse error, but parsing succeeded");
    assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    
    // Check error messages contain meaningful information
    let error_msg = &reporter.errors[0].1;
    assert!(error_msg.contains("';'") || error_msg.contains("semicolon"), 
            "Error should mention missing semicolon, but got: {}", error_msg);
}

#[test]
fn test_unmatched_parentheses() {
    let (result, reporter) = parse("(1 + 2;");
    assert!(result.is_none(), "Expected parse error, but parsing succeeded");
    assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    
    let error_msg = &reporter.errors[0].1;
    assert!(error_msg.contains("parenthesis") || error_msg.contains("')"), 
            "Error should mention missing parenthesis, but got: {}", error_msg);
}

#[test]
fn test_invalid_assignment_target() {
    let (result, reporter) = parse("1 + 2 = 3;");
    assert!(result.is_none(), "Expected parse error, but parsing succeeded");
    assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    
    let error_msg = &reporter.errors[0].1;
    assert!(error_msg.contains("assignment target") || error_msg.contains("Invalid assignment"), 
            "Error should mention invalid assignment target, but got: {}", error_msg);
}

#[test]
fn test_missing_expression() {
    let (result, reporter) = parse("print ;");
    assert!(result.is_none(), "Expected parse error, but parsing succeeded");
    assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    
    let error_msg = &reporter.errors[0].1;
    assert!(error_msg.contains("expression") || error_msg.contains("Expected"), 
            "Error should mention missing expression, but got: {}", error_msg);
}

#[test]
fn test_incomplete_ternary() {
    let (result, reporter) = parse("true ? 1;");
    assert!(result.is_none(), "Expected parse error, but parsing succeeded");
    assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    
    let error_msg = &reporter.errors[0].1;
    assert!(error_msg.contains("':'") || error_msg.contains("Expected"), 
            "Error should mention missing ':' in ternary, but got: {}", error_msg);
}

#[test]
fn test_invalid_variable_declaration() {
    // The scanner might not catch this, so let's use a syntax that will definitely fail parsing
    let (result, reporter) = parse("var ; // missing identifier");
    assert!(result.is_none() || !reporter.errors.is_empty(), 
            "Expected parse error or reported error");
    
    if result.is_some() {
        // If parsing somehow succeeded, we should see errors from scanner
        assert!(!reporter.errors.is_empty(), "Expected error to be reported");
    }
}

#[test]
fn test_unterminated_string() {
    // This tests scanner error handling, but still important for overall error reporting
    let (result, reporter) = parse("print \"unterminated;");
    assert!(result.is_none() || reporter.errors.len() > 0);
    
    // Check if any error messages relate to the string
    let has_string_error = reporter.errors.iter().any(|(_, msg)| 
        msg.contains("string") || msg.contains("unterminated"));
    assert!(has_string_error, "Error should mention unterminated string");
}