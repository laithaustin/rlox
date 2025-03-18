mod token;
use token::Token;
use token::TokenType;

// use rlox::Lox::error;

struct Scanner{
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    fn new(source: String) -> Self {
       Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        } 
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        // process tokens one by one
        while(!self.at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        // append a EOF to stream
        self.tokens.push(Token::
            new(
                TokenType::EOF,
                String::from(""),
                self.line,
                None
            )
        )
    }

    fn at_end(&self) -> bool {
        return self.current > self.source.len(); 
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);        
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, literal));
    }

    fn scan_token(&mut self) {
        char c = self.advance();
        match c {
            // single chars
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),

            // double chars
            '!' => {
                if self.match('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                if self.match('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '<' => {
                if self.match('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '>' => {
                if self.match('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            '/' => {
                if self.match('/') {
                    // A comment goes until the end of the line.
                    while (self.peek() != '\n' && !self.at_end()) {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },

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
                main::Lox::error(self.line, "Unexpected character.");
            }
        }
    }
}
