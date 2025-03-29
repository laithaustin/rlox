use crate::compiler::astPrinter::AstPrinter;
use crate::compiler::expr::{Binary, Expr, Grouping, Literal, Object, Unary};
use crate::compiler::token::TokenType;
use crate::compiler::{ErrorReporter, Scanner, Token};

// The essential grammar for lox is as follows (low to high precedence):
// expression -> equality;
// equality -> comparison ( ( "!=" | "==" ) comparison )*;
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

    // implementing the grammar rules as methods
    pub fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ()> {
        // Implementation for equality parsing will go here
        let mut expr: Expr = self.comparison()?;
        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.advance().unwrap().clone();
            let right = self.comparison()?;
            // Create a Binary expression node wrapped in Expr enum
            expr = Expr::Binary(Box::new(Binary {
                left: Box::new(expr), // Box the left expression
                operator,
                right: Box::new(right), // Box the right expression
            }));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, ()> {
        // let's just return some dummy value for now
        // Construct Literal and wrap in Expr enum
        let expr: Expr = Expr::Literal(Literal {
            value: Object::Number(0.0),
        });
        return Ok(expr);
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        // Parser implementation will go here
        let result: Result<Expr, ()> = self.expression(); // Changed type annotation
        match result {
            Ok(expr) => {
                // Successfully parsed the expression
                println!("Parsed expression successfully: {:?}", expr); // Added debug print
                // let's print the parsed expression for now
                let mut printer = AstPrinter;
                // get the type of the expression
                let printed: String = expr.accept(&printer); // Assuming accept method is
                println!("Printed expression: {}", printed); // Added debug print
            }
            Err(_) => {
                // Handle parsing error
                self.error_reporter.error(0, "Failed to parse expression");
                return Err(());
            }
        }

        Ok(())
    }
}
