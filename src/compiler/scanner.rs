use crate::compiler::error::ErrorReporter;
use crate::compiler::expr::Object;
use crate::compiler::token::Token;
use crate::compiler::token::TokenType;

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

    fn peek_next(&self) -> char {
        // Safety check
        if self.current >= self.source.len() {
            return '\0';
        }

        // Get a direct char iterator to the current position
        let mut char_iter = self.source[self.current..].chars();

        // Get current character and advance the iterator to next one
        let _ = char_iter.next();

        // Return the next character or null if there isn't one
        match char_iter.next() {
            Some(ch) => ch,
            None => '\0',
        }
    }

    fn string(&mut self) {
        self.start = self.current - 1; // Include the opening quote in lexeme
        let mut s = String::new();
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.peek() == '\\' {
                self.advance(); // consume the backslash
                match self.advance() {
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    _ => self
                        .error_reporter
                        .error(self.line, "Invalid escape sequence."),
                }
                continue;
            }
            s.push(self.advance());
        }

        if self.at_end() {
            self.error_reporter.error(self.line, "Unterminated string.");
            return;
        }

        // push new token type with literal
        self.advance(); // consume closing "
        self.add_token_literal(TokenType::STRING, Some(s));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for decimal part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the dot
            while self.peek().is_ascii_digit() {
                self.advance();
            }
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

        let text = self.source[self.start..self.current].to_string();
        let lowercase_text = text.to_lowercase();
        let token: TokenType = lowercase_text.parse().unwrap_or(TokenType::IDENTIFIER);
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
            '?' => self.add_token(TokenType::QUEST),
            ':' => self.add_token(TokenType::COLON),

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
                } else if self.check('*') {
                    // Multiline comment
                    let mut nesting = 1;
                    while nesting > 0 && !self.at_end() {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            nesting -= 1;
                            self.advance(); // consume *
                            self.advance(); // consume /
                        } else if self.peek() == '/' && self.peek_next() == '*' {
                            nesting += 1;
                            self.advance(); // consume /
                            self.advance(); // consume *
                        } else {
                            if self.peek() == '\n' {
                                self.line += 1;
                            }
                            self.advance();
                        }
                    }
                    if nesting > 0 {
                        self.error_reporter
                            .error(self.line, "Unterminated multiline comment");
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
