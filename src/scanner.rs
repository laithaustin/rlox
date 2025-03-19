use crate::token::Token;
use crate::token::TokenType;

use crate::error::ErrorReporter;

pub struct Scanner<'a> {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_reporter: &'a mut dyn ErrorReporter
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, error_reporter: &'a mut dyn ErrorReporter) -> Self {
       Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter
        } 
    }

    pub fn scan_tokens(&mut self) {
        // process tokens one by one
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // append a EOF to stream
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), self.line, None));
    }

    fn at_end(&self) -> bool {
        return self.current >= self.source.len(); 
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);        
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, literal));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        // utf8 encoded meaning chars can be more than 1 byte
        // TODO: highly unefficient O(n) operation
        return self.source.chars().nth(self.current - 1).unwrap();
    }

    fn check(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // single chars
            '(' => self.add_token(TokenType::LPAREN),
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LBRACE),
            '}' => self.add_token(TokenType::RBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),

            // double chars
            '!' => {
                if self.check('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                if self.check('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '<' => {
                if self.check('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '>' => {
                if self.check('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            /* '/' => {
                if self.check('/') {
                    // A comment goes until the end of the line.
                    while (self.peek() != '\n' && !self.at_end()) {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }, */

            // ignore whitespaces
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            // '"' => self.string(),

            // handle numbers/identifiers
            _ => {
                /* if c.is_digit() {
                    self.number();
                } else if c.is_alpha() {
                    self.identifier();
                } else {
                    // Lox.error(line, "Unexpected character.");
                } */
                // print error message
                self.error_reporter.error(self.line, "Unexpected character.");
            }
        }
    }
}
