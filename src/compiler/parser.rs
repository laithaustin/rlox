use crate::compiler::astPrinter::AstPrinter;
use crate::compiler::expr::{Binary, Expr, Grouping, Literal, Object, Ternary, Unary};
use crate::compiler::token::TokenType;
use crate::compiler::{ErrorReporter, Scanner, Token};

// The essential grammar for lox is as follows (low to high precedence):
// expression -> equality;
// equality -> ternary ( ( "!=" | "==" ) ternary)*;
// ternary -> comparison ( ("?") expression (":") ternary)*; //NOTE: ternary operator is RIGHT
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*;
// term -> factor ( ( "-" | "+" ) factor )*;
// factor -> unary ( ( "/" | "*" ) unary )*;
// unary -> ( "!" | "-" ) unary | primary;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    error_reporter: &'a mut dyn ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_reporter: &'a mut dyn ErrorReporter) -> Self {
        Self {
            tokens,
            current: 0,
            error_reporter,
        }
    }

    // implementing the grammar rules as methods
    pub fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ()> {
        // Implementation for equality parsing will go here
        let mut expr: Expr = self.ternary()?;
        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.advance().unwrap().clone();
            let right = self.ternary()?;
            // Create a Binary expression node wrapped in Expr enum
            expr = Expr::Binary(Box::new(Binary {
                left: Box::new(expr), // Box the left expression
                operator,
                right: Box::new(right), // Box the right expression
            }));
        }

        Ok(expr)
    }

    pub fn ternary(&mut self) -> Result<Expr, ()> {
        // ternary -> comparison ( ("?") expression (":") ternary)*;
        let mut expr: Expr = self.comparison()?;
        while self.match_token(&[TokenType::QUEST]) {
            let _ = self.advance(); // consume '?'  
            let left = self.expression()?; // parse the left expression
            // check for ':' token
            if !self.match_token(&[TokenType::COLON]) {
                self.error_reporter.error(0, "Expected ':' after '?'");
                return Err(());
            }
            _ = self.advance(); // consume ':'
            let right = self.ternary()?; // parse the right expression
            expr = Expr::Ternary(Box::new(Ternary {
                condition: Box::new(expr),
                true_branch: Box::new(left),
                false_branch: Box::new(right),
            }));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, ()> {
        // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*;

        // let's just return some dummy value for now
        // Construct Literal and wrap in Expr enum
        let mut expr: Expr = self.term()?; // Start with a term
        //
        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator: Token = self.advance().unwrap().clone(); // Get the operator token
            let right = self.term()?; // Get the next term

            // Create a Binary expression node wrapped in Expr enum
            expr = Expr::Binary(Box::new(Binary {
                left: Box::new(expr), // Box the left expression
                operator,
                right: Box::new(right), // Box the right expression
            }));
        }

        Ok(expr) // Return the final expression
    }

    pub fn term(&mut self) -> Result<Expr, ()> {
        // term -> factor ( ( "-" | "+" ) factor )*;

        let mut expr: Expr = self.factor()?;
        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.advance().unwrap().clone(); // Get the operator token
            let right = self.factor()?; // Get the next factor

            // Create a Binary expression node wrapped in Expr enum
            expr = Expr::Binary(Box::new(Binary {
                left: Box::new(expr), // Box the left expression
                operator,
                right: Box::new(right), // Box the right expression
            }));
        }

        Ok(expr) // Return the final expression
    }

    pub fn factor(&mut self) -> Result<Expr, ()> {
        // factor -> unary ( ( "/" | "*" ) unary )*;

        let mut expr: Expr = self.unary()?;
        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.advance().unwrap().clone(); // Get the operator token
            let right = self.unary()?; // Get the next unary expression

            // Create a Binary expression node wrapped in Expr enum
            expr = Expr::Binary(Box::new(Binary {
                left: Box::new(expr), // Box the left expression
                operator,
                right: Box::new(right), // Box the right expression
            }));
        }

        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, ()> {
        // unary -> ( "!" | "-" ) unary | primary;

        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.advance().unwrap().clone(); // Get the operator token
            let right = self.unary()?; // Get the next unary expression

            // Create a Unary expression node wrapped in Expr enum
            return Ok(Expr::Unary(Box::new(Unary {
                operator,
                right: Box::new(right), // Box the right expression
            })));
        }

        // If not a unary operator, parse as primary
        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, ()> {
        // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")";

        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let token: Token = self.advance().unwrap().clone(); // Get the token
            // Create a Literal expression node wrapped in Expr enum
            return match token.token_type {
                TokenType::NUMBER => {
                    let value: f64 = token.lexeme.parse().unwrap(); // Parse the number
                    Ok(Expr::Literal(Literal {
                        value: Object::Number(value), // Wrap in Object::Number
                    }))
                }
                TokenType::STRING => {
                    let value: String = token.lexeme.clone(); // Clone the string
                    Ok(Expr::Literal(Literal {
                        value: Object::String(value), // Wrap in Object::String
                    }))
                }
                _ => {
                    // This should not happen as we already checked for NUMBER and STRING
                    self.error_reporter
                        .error(0, "Unexpected token type in primary");
                    Err(()) // Error if unexpected token type
                }
            };
        }

        if self.match_token(&[TokenType::TRUE]) {
            self.advance().unwrap(); // Consume the token
            return Ok(Expr::Literal(Literal {
                value: Object::Boolean(true), // Wrap in Object::Bool
            }));
        }

        if self.match_token(&[TokenType::FALSE]) {
            self.advance().unwrap(); // Consume the token
            return Ok(Expr::Literal(Literal {
                value: Object::Boolean(false), // Wrap in Object::Bool
            }));
        }

        if self.match_token(&[TokenType::NIL]) {
            self.advance().unwrap(); // Consume the token
            return Ok(Expr::Literal(Literal {
                value: Object::Nil, // Wrap in Object::Bool
            }));
        }

        if self.match_token(&[TokenType::LPAREN]) {
            self.advance().unwrap(); // Consume '('
            let expr = self.expression()?; // Parse the inner expression

            if !self.match_token(&[TokenType::RPAREN]) {
                self.error_reporter.error(0, "Expected closing parenthesis");
                return Err(()); // Error if no closing parenthesis
            }
            self.advance().unwrap(); // Consume ')'

            return Ok(Expr::Grouping(Box::new(Grouping {
                expression: Box::new(expr),
            }))); // Grouping expression
        }

        // If none of the above, it's an error
        self.error_reporter.error(0, "Unexpected token");
        Err(())
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        let expr = self.expression()?;

        // Debug printing
        println!("Parsed expression successfully: {:?}", expr);
        let printer = AstPrinter;
        let printed: String = expr.accept(&printer);
        println!("Printed expression: {}", printed);

        Ok(expr)
    }

    pub fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                return true;
            }
        }
        false
    }

    pub fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *token_type;
    }

    pub fn advance(&mut self) -> Result<&Token, String> {
        if self.is_at_end() {
            self.error_reporter.error(0, "Unexpected end of input");
            return Err("Unexpected end of input".to_string());
        }
        self.current += 1;
        // let's debug print the current token
        Ok(&self.tokens[self.current - 1])
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.tokens.len();
    }

    pub fn peek(&mut self) -> &Token {
        if self.is_at_end() {
            // check if last token is EOF - if so return it else error
            if self
                .tokens
                .last()
                .map_or(false, |t| t.token_type == TokenType::EOF)
            {
                return &self.tokens[self.tokens.len() - 1];
            }
            // Error: unexpected end of input
            self.error_reporter.error(0, "Unexpected end of input");
        }
        &self.tokens[self.current]
    }
}
