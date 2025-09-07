use crate::common::TestErrorReporter;
use lox::compiler::error::{ErrorReporter, Result};
use lox::compiler::interpreter::Interpreter;
use lox::compiler::parser::Parser;
use lox::compiler::resolver::Resolver;
use lox::compiler::scanner::Scanner;
use std::cell::RefCell;
use std::rc::Rc;

// Helper function to execute a source string and return any runtime errors
fn execute(source: &str) -> (Result<()>, TestErrorReporter) {
    let mut reporter = TestErrorReporter::new();

    // Scanning phase - scope scanner to release reporter borrow
    let tokens = {
        let mut scanner = Scanner::new(source.to_string(), &mut reporter);
        scanner.scan_tokens();
        scanner.tokens.clone()
    };

    if reporter.has_errors() {
        return (Ok(()), reporter);
    }

    // Parsing phase
    let mut parser = Parser::new(&tokens);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(_) => return (Ok(()), reporter),
    };

    if reporter.has_errors() {
        return (Ok(()), reporter);
    }

    // Run resolver and interpreter
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let resolver = Resolver::new(interpreter.clone());

    // Resolve first
    resolver.resolve_statements(&statements);

    // Check for resolver errors (not warnings)
    let has_real_errors = resolver
        .errors
        .borrow()
        .iter()
        .any(|e| e.kind != lox::compiler::error::LoxErrorKind::Warning);

    if has_real_errors {
        // Find the first non-warning error
        let resolver_error = resolver
            .errors
            .borrow()
            .iter()
            .find(|e| e.kind != lox::compiler::error::LoxErrorKind::Warning)
            .cloned()
            .unwrap();
        return (Err(resolver_error), reporter);
    }

    // Interpretation phase
    let result = interpreter.borrow_mut().interpret(statements);

    (result, reporter)
}

#[test]
fn test_simple_function_declaration() {
    let source = r#"
        fun greet() {
            print "Hello, World!";
        }
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_function_call_no_args() {
    let source = r#"
        fun sayHello() {
            print "Hello!";
        }
        sayHello();
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_function_with_parameters() {
    let source = r#"
        fun greet(name) {
            print "Hello, " + name + "!";
        }
        greet("Alice");
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    if let Err(e) = &result {
        eprintln!("Test failed with error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_with_multiple_parameters() {
    let source = r#"
        fun add(a, b) {
            print a + b;
        }
        add(3, 4);
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    if let Err(e) = &result {
        eprintln!("Test failed with error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_scope_isolation() {
    let source = r#"
        var global = "global";
        fun test() {
            var local = "local";
            print local;
        }
        test();
        print global;
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_function_parameter_shadowing() {
    let source = r#"
        var x = "global";
        fun test(x) {
            print x;
        }
        test("parameter");
        print x;
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_function_arity_error_too_few_args() {
    let source = r#"
        fun test(a, b) {
            print a + b;
        }
        test(1);
    "#;

    let (result, _) = execute(source);
    assert!(result.is_err());
}

#[test]
fn test_function_arity_error_too_many_args() {
    let source = r#"
        fun test(a) {
            print a;
        }
        test(1, 2, 3);
    "#;

    let (result, _) = execute(source);
    assert!(result.is_err());
}

#[test]
fn test_call_non_function() {
    let source = r#"
        var notAFunction = "hello";
        notAFunction();
    "#;

    let (result, _) = execute(source);
    assert!(result.is_err());
}

#[test]
fn test_undefined_function() {
    let source = r#"
        undefinedFunction();
    "#;

    let (result, _) = execute(source);
    assert!(result.is_err());
}

#[test]
fn test_function_with_expressions() {
    let source = r#"
        fun calculate(x, y) {
            var sum = x + y;
            var product = x * y;
            print "Sum: " + sum;
            print "Product: " + product;
        }
        calculate(5, 3);
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    if let Err(e) = &result {
        eprintln!("Test failed with error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_with_conditionals() {
    let source = r#"
        fun max(a, b) {
            if (a > b) {
                print a;
            } else {
                print b;
            }
        }
        max(10, 5);
        max(3, 8);
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    if let Err(e) = &result {
        eprintln!("Test failed with error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_with_loops() {
    let source = r#"
        fun countdown(n) {
            while (n > 0) {
                print n;
                n = n - 1;
            }
            print "Done!";
        }
        countdown(3);
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    if let Err(e) = &result {
        eprintln!("Test failed with error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_native_function_clock() {
    let source = r#"
        var time = clock();
        print "Current time: " + time;
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_function_closure_basic() {
    let source = r#"
        var global = "I am global";
        fun test() {
            print global;
        }
        test();
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_function_declarations() {
    let source = r#"
        fun first() {
            print "First function";
        }

        fun second() {
            print "Second function";
        }

        first();
        second();
        first();
    "#;

    let (result, reporter) = execute(source);
    reporter.assert_no_errors();
    assert!(result.is_ok());
}
