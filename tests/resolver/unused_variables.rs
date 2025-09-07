use crate::common::TestErrorReporter;
use lox::compiler::{Interpreter, Parser, Resolver, Scanner};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_unused_local_variable() {
    let source = r#"
    {
        var x = 10;  // x is declared but never used
        var y = 20;
        print y;     // y is used
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

    // Check that resolver found the unused variable error
    let errors = resolver.errors.borrow();

    // We should have at least one error about unused variable 'x'
    let has_unused_error = errors.iter().any(|e| {
        e.message.contains("x")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(
        has_unused_error,
        "Should detect unused variable 'x'. Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_all_variables_used() {
    let source = r#"
    {
        var x = 10;
        var y = 20;
        print x + y;  // Both variables are used
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

    // Check that resolver found no unused variable errors
    let errors = resolver.errors.borrow();

    let has_unused_error = errors
        .iter()
        .any(|e| e.message.contains("unused") || e.message.contains("never used"));

    assert!(
        !has_unused_error,
        "Should not detect any unused variables. Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_variable_used_in_nested_scope() {
    let source = r#"
    {
        var x = 10;
        {
            print x;  // x is used in nested scope
        }
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

    // Check that resolver does NOT report x as unused
    let errors = resolver.errors.borrow();

    let has_unused_x = errors.iter().any(|e| {
        e.message.contains("x")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(
        !has_unused_x,
        "Should NOT report 'x' as unused when used in nested scope. Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_variable_assigned_but_not_read() {
    let source = r#"
    {
        var x = 10;
        x = 20;  // Assignment without read - still counts as unused in many linters
        var y = 30;
        print y;
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

    let errors = resolver.errors.borrow();

    // Assignment doesn't mark as "used" in visit_assign, only reads do
    // So x should be detected as unused
    let has_unused_x = errors.iter().any(|e| {
        e.message.contains("x")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(
        has_unused_x,
        "Should detect 'x' as unused (write-only variable). Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_shadowed_variable_usage() {
    let source = r#"
    {
        var x = 10;
        print x;  // Outer x is used
        {
            var x = 20;  // Inner x shadows outer x
            // Inner x is NOT used
        }
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

    let errors = resolver.errors.borrow();

    // Should detect that inner 'x' is unused
    let has_unused_inner_x = errors.iter().any(|e| {
        e.message.contains("x")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(
        has_unused_inner_x,
        "Should detect that inner shadowing 'x' is unused. Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_function_parameter_usage() {
    let source = r#"
    fun test(a, b) {
        print a;  // Only 'a' is used, 'b' is not
    }
    test(1, 2);
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

    // Check for unused parameter 'b'
    let errors = resolver.errors.borrow();

    let has_unused_param = errors.iter().any(|e| {
        e.message.contains("b")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    // Function parameters are often allowed to be unused (e.g., interface compliance)
    // But let's see what the resolver does
    if has_unused_param {
        println!("Resolver detects unused parameter 'b' (strict mode)");
    } else {
        println!("Resolver allows unused parameters (common practice)");
    }
}

#[test]
fn test_multiple_unused_variables() {
    let source = r#"
    {
        var a = 1;
        var b = 2;
        var c = 3;
        var d = 4;
        print d;  // Only d is used
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

    let errors = resolver.errors.borrow();

    // Should detect a, b, c as unused
    let unused_a = errors.iter().any(|e| {
        e.message.contains("'a'")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });
    let unused_b = errors.iter().any(|e| {
        e.message.contains("'b'")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });
    let unused_c = errors.iter().any(|e| {
        e.message.contains("'c'")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });
    let unused_d = errors.iter().any(|e| {
        e.message.contains("'d'")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(unused_a, "Should detect 'a' as unused");
    assert!(unused_b, "Should detect 'b' as unused");
    assert!(unused_c, "Should detect 'c' as unused");
    assert!(!unused_d, "Should NOT detect 'd' as unused");
}

#[test]
fn test_variable_used_in_condition() {
    let source = r#"
    {
        var flag = true;
        if (flag) {  // flag is used in condition
            print "yes";
        }
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

    let errors = resolver.errors.borrow();

    let has_unused_flag = errors.iter().any(|e| {
        e.message.contains("flag")
            && (e.message.contains("unused") || e.message.contains("never used"))
    });

    assert!(
        !has_unused_flag,
        "Should NOT report 'flag' as unused when used in condition. Found {} errors: {:?}",
        errors.len(),
        errors.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
}
