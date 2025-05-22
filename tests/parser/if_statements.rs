use crate::common::TestErrorReporter;
use lox::compiler::error::{Result, ErrorReporter};
use lox::compiler::interpreter::Interpreter;
use lox::compiler::parser::Parser;
use lox::compiler::scanner::Scanner;

// Helper function to execute a source string and return any runtime errors
fn execute(source: &str) -> (Result<()>, TestErrorReporter) {
    let mut reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut reporter);
    scanner.scan_tokens();
    
    let mut parser = Parser::new(&scanner.tokens);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => {
            // If parsing fails, return the parse error to avoid confusing test failures
            return (Err(e), reporter);
        }
    };
    
    // Execute the parsed statements
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(statements);
    
    // Record runtime errors in the reporter for later inspection
    if let Err(err) = &result {
        reporter.runtime_error(err);
    }
    
    (result, reporter)
}

#[test]
fn test_if_basic() {
    let (result, reporter) = execute(r#"
        var x = 10;
        if (x > 5) {
            print "greater";
        }
    "#);
    
    assert!(result.is_ok(), "Basic if statement should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_else_basic() {
    let (result, reporter) = execute(r#"
        var x = 3;
        if (x > 5) {
            print "greater";
        } else {
            print "lesser or equal";
        }
    "#);
    
    assert!(result.is_ok(), "If-else statement should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_condition_false() {
    let (result, reporter) = execute(r#"
        var x = 3;
        if (x > 5) {
            print "should not print";
        }
        print "done";
    "#);
    
    assert!(result.is_ok(), "If statement with false condition should skip the body");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_truthy_values() {
    // Test that non-boolean values are properly coerced to boolean
    let (result, reporter) = execute(r#"
        // These should all be truthy
        if (1) print "number is truthy";
        if ("hello") print "string is truthy";
        if (4.5) print "decimal is truthy";
        
        // These should be falsy
        if (nil) print "nil should be falsy - won't print";
        if (false) print "false should be falsy - won't print";
    "#);
    
    assert!(result.is_ok(), "Truthy/falsy values should work in if conditions");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_syntax_error_missing_paren() {
    let (result, _reporter) = execute("if (x > 5 print x;");
    
    assert!(result.is_err(), "Missing closing parenthesis should cause error");
    
    if let Err(error) = result {
        assert!(error.message.contains(")") || error.message.contains("parenthesis"),
                "Error should mention missing parenthesis: {}", error.message);
    }
}

#[test]
fn test_if_syntax_error_missing_condition() {
    let (result, _reporter) = execute("if () print x;");
    
    assert!(result.is_err(), "Missing condition should cause error");
    
    if let Err(error) = result {
        assert!(error.message.contains("expression") || error.message.contains("condition"),
                "Error should mention missing condition: {}", error.message);
    }
}

#[test]
fn test_nested_if_statements() {
    let (result, reporter) = execute(r#"
        var x = 10;
        var y = 5;
        
        if (x > 5) {
            if (y < 10) {
                print "both conditions true";
            } else {
                print "only outer condition true";
            }
        } else {
            print "outer condition false";
        }
    "#);
    
    assert!(result.is_ok(), "Nested if statements should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_else_chain() {
    let (result, reporter) = execute(r#"
        var score = 85;
        
        if (score >= 90) {
            print "A";
        } else if (score >= 80) {
            print "B";
        } else if (score >= 70) {
            print "C";
        } else {
            print "D";
        }
    "#);
    
    assert!(result.is_ok(), "If-else-if chain should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_block_scope() {
    let (result, reporter) = execute(r#"
        var x = 10;
        
        if (true) {
            var x = 20;  // Shadows outer x
            print x;     // Should print 20
        }
        
        print x;         // Should print 10
    "#);
    
    assert!(result.is_ok(), "If statement block should have proper scope");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_if_expression_evaluation() {
    let (result, reporter) = execute(r#"
        var a = 5;
        var b = 10;
        
        if (a + b > 12) {
            print "sum greater than 12";
        } else {
            print "sum less than or equal to 12";
        }
    "#);
    
    assert!(result.is_ok(), "If statement with complex condition should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}