use crate::compiler::token::Token;
use crate::compiler::token::TokenType;

use crate::compiler::error::ErrorReporter;

pub struct Scanner<'a> {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_reporter: &'a mut dyn ErrorReporter,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, error_reporter: &'a mut dyn ErrorReporter) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter,
        }
    }

    pub fn scan_tokens(&mut self) {
        // process tokens one by one
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // append a EOF to stream
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            self.line,
            None,
        ));
    }

    fn at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += c.len_utf8();
        c
    }

    fn check(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += expected.len_utf8();
        return true;
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn string(&mut self) {
        let mut s = String::new();
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            s.push(self.advance());
        }

        if self.at_end() {
            self.error_reporter.error(self.line, "Unterminated string");
            return;
        }

        // push new token type with literal
        self.advance(); // consume closing "
        self.add_token_literal(TokenType::STRING, Some(s));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() || self.peek() == '.' {
            self.advance();
        }

        // parse number
        let num_str = self.source[self.start..self.current].to_string();
        let num: f64 = num_str.parse().unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(num.to_string()));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = self.source[self.start..self.current]
            .to_string()
            .to_lowercase();
        let token: TokenType = text.parse().unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token);
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
            }
            '=' => {
                if self.check('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.check('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.check('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.check('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }

            // ignore whitespaces
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),

            // handle numbers/identifiers
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    self.error_reporter
                        .error(self.line, &format!("Unexpected character '{}'", c));
                }
            }
        }
    }
}
