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
fn test_logical_and_basic() {
    let (result, reporter) = execute(r#"
        var a = true;
        var b = true;
        if (a and b) {
            print "Both true";
        } else {
            print "Not both true";
        }
    "#);
    
    assert!(result.is_ok(), "Basic AND operation should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_or_basic() {
    let (result, reporter) = execute(r#"
        var a = false;
        var b = true;
        if (a or b) {
            print "At least one true";
        } else {
            print "Both false";
        }
    "#);
    
    assert!(result.is_ok(), "Basic OR operation should execute successfully");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_and_short_circuit() {
    let (result, reporter) = execute(r#"
        var a = false;
        var b = "right side not evaluated";
        
        // This should short-circuit at 'a' and not evaluate 'b'
        var result = a and b;
        
        // The result should be false (the left operand's value)
        print result;
    "#);
    
    assert!(result.is_ok(), "AND short-circuit should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_or_short_circuit() {
    let (result, reporter) = execute(r#"
        var a = true;
        var b = "right side not evaluated";
        
        // This should short-circuit at 'a' and not evaluate 'b'
        var result = a or b;
        
        // The result should be true (the left operand's value)
        print result;
    "#);
    
    assert!(result.is_ok(), "OR short-circuit should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_and_falsy_values() {
    let (result, reporter) = execute(r#"
        // These should all evaluate to falsy in AND expressions
        var result1 = false and true;    // false
        var result2 = nil and true;      // nil
        
        // These should all evaluate to the right operand
        var result3 = true and "hello";  // "hello"
        var result4 = true and 42;       // 42
        var result5 = true and false;    // false
        
        print result1;
        print result2;
        print result3;
        print result4;
        print result5;
    "#);
    
    assert!(result.is_ok(), "AND with various falsy values should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_or_truthy_values() {
    let (result, reporter) = execute(r#"
        // These should all evaluate to truthy in OR expressions
        var result1 = true or false;     // true
        var result2 = "hello" or false;  // "hello"
        var result3 = 42 or false;       // 42
        
        // These should all evaluate to the right operand
        var result4 = false or true;     // true
        var result5 = nil or "fallback"; // "fallback"
        var result6 = false or false;    // false
        
        print result1;
        print result2;
        print result3;
        print result4;
        print result5;
        print result6;
    "#);
    
    assert!(result.is_ok(), "OR with various truthy values should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_operator_chaining() {
    let (result, reporter) = execute(r#"
        // Chains of the same operator
        var result1 = true and true and true;      // true
        var result2 = true and false and true;     // false (short-circuits at 'false')
        var result3 = false or false or true;      // true
        var result4 = true or false or false;      // true (short-circuits at first 'true')
        
        print result1;
        print result2;
        print result3;
        print result4;
    "#);
    
    assert!(result.is_ok(), "Chained logical operators should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_operator_precedence() {
    let (result, reporter) = execute(r#"
        // AND has higher precedence than OR
        var result1 = false or true and false;   // false (like: false or (true and false))
        var result2 = (false or true) and false; // false
        var result3 = true or false and true;    // true (like: true or (false and true))
        
        print result1;
        print result2;
        print result3;
    "#);
    
    assert!(result.is_ok(), "Logical operator precedence should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_with_comparison() {
    let (result, reporter) = execute(r#"
        var a = 5;
        var b = 10;
        
        // Comparison operators have higher precedence than logical operators
        var result1 = a < 10 and b > 5;    // true
        var result2 = a > 10 or b < 20;    // true
        var result3 = a < 3 or b < 5;      // false
        
        print result1;
        print result2;
        print result3;
    "#);
    
    assert!(result.is_ok(), "Logical operators with comparisons should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_in_if_conditions() {
    let (result, reporter) = execute(r#"
        var a = 5;
        var b = 10;
        
        // Using logical operators in if conditions
        if (a > 0 and b > 0) {
            print "Both positive";
        }
        
        if (a > 100 or b > 5) {
            print "At least one condition true";
        }
        
        if (a < 0 or b < 0) {
            print "This should not print";
        } else {
            print "Both non-negative";
        }
    "#);
    
    assert!(result.is_ok(), "Logical operators in if conditions should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_complex_expressions() {
    let (result, reporter) = execute(r#"
        var a = 5;
        var b = 10;
        var c = 15;
        
        // Complex expressions with logical operators, comparisons, and numeric operations
        var result1 = (a + 5 > b) or (b + 5 <= c);     // true
        var result2 = (a + b > c) and (a < b);         // true
        var result3 = (a > b) or (b > c) or (a + b < c); // false
        
        print result1;
        print result2;
        print result3;
    "#);
    
    assert!(result.is_ok(), "Complex expressions with logical operators should work correctly");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}

#[test]
fn test_logical_error_non_boolean_condition() {
    // Logical operators should work with any values, not just booleans
    let (result, reporter) = execute(r#"
        // These should all be valid, even though they're not boolean values
        var result1 = "hello" and 42;    // 42
        var result2 = 0 or "fallback";   // "fallback"
        
        print result1;
        print result2;
    "#);
    
    assert!(result.is_ok(), "Logical operators should work with non-boolean values");
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}