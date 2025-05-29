use std::{
    env, fs,
    io::{self, BufRead, Write},
};

mod compiler;

use compiler::{ErrorReporter, Interpreter, LoxError, LoxErrorKind, Parser, Scanner};

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl ErrorReporter for Lox {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn runtime_error(&mut self, error: &LoxError) {
        eprintln!("{}", error);
        self.had_runtime_error = true;
    }
}

impl Lox {
    fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }

    fn run(&mut self, source: String) {
        // first phase: tokenize the input
        let mut scanner = Scanner::new(source, self);
        scanner.scan_tokens();
        // dummy tokens for testing (need to use type annotations)
        // let tokens = vec!["(", ")", "{", "}", ",", ".", "-", "+", ";", "*", "!"];

        // Debug: print tokens (disabled for now)
        // for token in &scanner.tokens {
        //     println!("{:?}", token);
        // }

        // second phase: parse the tokens
        let mut parser = Parser::new(&scanner.tokens);
        match parser.parse() {
            Ok(ast) => {
                let mut interpreter = Interpreter::new();
                match interpreter.interpret(ast) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Runtime error: {}", e);
                        self.had_runtime_error = true;
                    }
                }
            }
            Err(error) => {
                // Display the parsing error
                eprintln!("Parse error: {}", error);
                self.had_error = true;
                return;
            }
        }
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut line = String::new();
            let bytes = stdin.lock().read_line(&mut line).unwrap();

            if bytes == 0 {
                break;
            }

            self.run(line);
            // prevent error from stopping the REPL
            self.had_error = false;
            self.had_runtime_error = false;
        }
    }

    fn run_file(&mut self, path: &str) {
        let content = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Could not read file '{}': {}", path, err);
            std::process::exit(65);
        });

        self.run(content);

        // Exit with different error codes for different error types
        if self.had_error {
            std::process::exit(65);
        }
        if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, message);
        self.had_error = true;
    }
}

fn main() {
    let mut lox = Lox::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}
