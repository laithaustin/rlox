use crate::common::TestErrorReporter;
use lox::compiler::{Interpreter, Parser, Scanner};

#[test]
fn test_interpret_simple_script() {
    let source = r#"
        fun add(a, b) {
            return a + b;
        }

        print add(2, 3);
        print "Hello, World!";
    "#;

    // Scan the source into tokens
    let mut reporter = TestErrorReporter::new();
    let tokens = {
        let mut scanner = Scanner::new(source.to_string(), &mut reporter);
        scanner.scan_tokens();
        scanner.tokens.clone()
    };

    // Parse the tokens into statements
    let mut parser = Parser::new(&tokens);
    let statements = parser
        .parse()
        .expect("Parser should succeed on valid source");

    // Interpret the statements
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(statements);

    assert!(result.is_ok(), "Interpreter failed: {:?}", result.err());
    reporter.assert_no_errors();
    reporter.assert_no_runtime_errors();
}
