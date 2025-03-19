use std::{env, fs, io::{self, BufRead, Write}, path::Path};

mod scanner;
mod token;
mod error;

use scanner::Scanner;
use token::TokenType;
use token::Token;
use error::ErrorReporter;

pub struct Lox {
    had_error: bool,
}

impl ErrorReporter for Lox {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        scanner.scan_tokens();
        // dummy tokens for testing (need to use type annotations)
        // let tokens = vec!["(", ")", "{", "}", ",", ".", "-", "+", ";", "*", "!"];
        
        for token in scanner.tokens {
            println!("{:?}", token);
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
        }
    }

    fn run_file(&mut self, path: &str) {
        let content = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Could not read file '{}': {}", path, err);
            std::process::exit(65);
        });

        self.run(content);

        if self.had_error {
            std::process::exit(65);
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


