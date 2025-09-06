use crate::common::*;
use lox::compiler::{ErrorReporter, Interpreter, Parser, Resolver, Scanner};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_return_at_top_level() {
    let source = r#"return "at top level";"#;

    let mut error_reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut error_reporter);
    scanner.scan_tokens();

    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse().expect("Parsing should succeed");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve the AST
    resolver.resolve_statements(&ast);

    // Check that resolver found the error
    let errors = resolver.errors.borrow();
    assert!(!errors.is_empty(), "Should have resolver errors");

    // Check that the error is about returning at top level
    let error_msg = errors[0].message.to_lowercase();
    assert!(
        error_msg.contains("return") && error_msg.contains("top"),
        "Error should mention return at top level, got: {}",
        error_msg
    );
}

#[test]
fn test_duplicate_variable_declaration() {
    let source = r#"
fun bad() {
    var a = "first";
    var a = "second";
}
"#;

    let mut error_reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut error_reporter);
    scanner.scan_tokens();

    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse().expect("Parsing should succeed");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve the AST
    resolver.resolve_statements(&ast);

    // Check that resolver found the error
    let errors = resolver.errors.borrow();
    assert!(
        !errors.is_empty(),
        "Should have resolver errors for duplicate variable"
    );

    // Check that the error is about duplicate variable
    let error_msg = errors[0].message.to_lowercase();
    assert!(
        error_msg.contains("already")
            || error_msg.contains("duplicate")
            || error_msg.contains("declared"),
        "Error should mention duplicate variable, got: {}",
        error_msg
    );
}

#[test]
fn test_closure_variable_resolution() {
    let source = r#"
var a = "global";
{
    fun showA() {
        print a;
    }
    showA();
    var a = "block";
    showA();
}
"#;

    let mut error_reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut error_reporter);
    scanner.scan_tokens();

    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse().expect("Parsing should succeed");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve the AST - this should succeed without errors
    resolver.resolve_statements(&ast);

    // Check that resolver didn't find any errors (this is valid code)
    let errors = resolver.errors.borrow();
    if !errors.is_empty() {
        panic!(
            "Resolver should not have errors for valid closure code, but got: {:?}",
            errors
        );
    }

    // Now try to interpret it to see if resolution worked correctly
    match interpreter.borrow_mut().interpret(ast) {
        Ok(_) => (), // Should succeed
        Err(e) => panic!(
            "Interpretation should succeed after resolution, but got error: {}",
            e
        ),
    }
}

#[test]
fn test_variable_used_before_declaration() {
    let source = r#"
var a = a;
"#;

    let mut error_reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut error_reporter);
    scanner.scan_tokens();

    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse().expect("Parsing should succeed");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve the AST
    resolver.resolve_statements(&ast);

    // Check that resolver found the error
    let errors = resolver.errors.borrow();
    assert!(
        !errors.is_empty(),
        "Should have resolver errors for using variable in its own initializer"
    );

    // Check that the error is about using variable before declaration
    let error_msg = errors[0].message.to_lowercase();
    assert!(
        error_msg.contains("initializer")
            || error_msg.contains("before")
            || error_msg.contains("own"),
        "Error should mention using variable in its own initializer, got: {}",
        error_msg
    );
}

#[test]
fn test_nested_function_scoping() {
    let source = r#"
fun outer() {
    var x = "outer";
    fun inner() {
        print x;
    }
    inner();
}
outer();
"#;

    let mut error_reporter = TestErrorReporter::new();
    let mut scanner = Scanner::new(source.to_string(), &mut error_reporter);
    scanner.scan_tokens();

    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse().expect("Parsing should succeed");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve the AST - this should succeed without errors
    resolver.resolve_statements(&ast);

    // Check that resolver didn't find any errors
    let errors = resolver.errors.borrow();
    if !errors.is_empty() {
        panic!(
            "Resolver should not have errors for valid nested function code, but got: {:?}",
            errors
        );
    }
}
