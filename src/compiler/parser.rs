use crate::compiler::expr::{
    Assign, Binary, Call, Expr, Grouping, Literal, Logical, Object, Ternary, Unary, Variable,
};
use crate::compiler::stmt::{Block, Expression, Function, IfStmt, Print, Stmt, Var, WhileStmt};
use crate::compiler::token::TokenType;
use crate::compiler::{LoxError, Result, Token};

// The essential grammar for lox is as follows (low to high precedence):
// program -> declaration* EOF
// declaration -> varStmt | statement
// varStmt -> "var" identifier ("=" expression)? ";"
// statement -> printStmt | exprStmt | whileStmt | forStmt | ifStmt | block | funcStmt ";"
// funcStmt -> "func" function;
// function -> Identifier "(" parameters? ")" block;
// parameters -> Identifier ("," Identifier)*
// forStmt -> "for" "(" (exprStmt | varStmt | ";") expression? ";" expression? ")" statement ";"
// whileStmt -> "while" "(" expression ")" statement ";"
// ifStmt -> if "(" expression ")" statement ( else statement )? ";"
// block -> "{" declaration* "}" ;
// printStmt -> "print" expression ";"
// exprStmt -> expression ";"
// expression -> IDENTIFIER "=" expression | logic_or;
// logic_or -> logic_and ("or" logic_and)*;
// logic_and -> equality ("and" equality)*;
// equality -> ternary ( ( "!=" | "==" ) ternary)*;
// ternary -> comparison ( ("?") expression (":") ternary)*; //NOTE: ternary operator is RIGHT
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*;
// term -> factor ( ( "-" | "+" ) factor )*;
// factor -> unary ( ( "/" | "*" ) unary )*;
// unary -> ( "!" | "-" ) unary | call ;
// call -> primary ( "(" arguments ")" ) ;
// arguments -> expression ( "," expression )*;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | identifier ;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    // implementing the grammar rules as methods
    pub fn expression(&mut self) -> Result<Expr> {
        let lval = self.logic_or()?; // get left expression

        if self.match_token(&[TokenType::EQUAL]) {
            let equals: Token = self.previous().clone();
            let val: Expr = self.expression()?;

            // check if lval is a variable type expression
            if let Expr::Variable(name) = lval {
                return Ok(Expr::Assign(Box::new(Assign {
                    name: name.name,
                    value: Box::new(val),
                })));
            }

            return Err(LoxError::new_parse(equals, "Invalid assignment target"));
        }

        Ok(lval)
    }

    pub fn fun_statement(&mut self) -> Result<Stmt> {
        self.function()
    }

    pub fn function(&mut self) -> Result<Stmt> {
        let name = self
            .consume(&TokenType::IDENTIFIER, "Expect function name.")?
            .clone();
        let params = self.fun_parameters()?;
        let body = self.block()?;
        Ok(Stmt::Function(Box::new(Function {
            name: Box::new(name),
            parameters: Box::new(params),
            body: Box::new(body),
        })))
    }

    pub fn fun_parameters(&mut self) -> Result<Vec<Token>> {
        let mut params = Vec::new();
        self.consume(&TokenType::LPAREN, "Expect '(' after function name.")?;

        if !self.check(&TokenType::RPAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(LoxError::new_parse(
                        self.peek().clone(),
                        "Cannot have more than 255 parameters.",
                    ));
                }
                params.push(
                    self.consume(&TokenType::IDENTIFIER, "Expect parameter name.")?
                        .clone(),
                );
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RPAREN, "Expect ')' after parameters.")?;

        Ok(params)
    }

    pub fn for_statement(&mut self) -> Result<Stmt> {
        // let's implement this via desugaring
        if self.match_token(&[TokenType::LPAREN]) {
            let initializer = match self.peek().token_type {
                TokenType::SEMICOLON => {
                    // consume semicolon
                    self.consume(
                        &TokenType::SEMICOLON,
                        "Expect ';' after for initialization.",
                    )?;
                    None
                }
                TokenType::VAR => {
                    self.advance();
                    Some(self.var_declar())
                }
                _ => Some(self.expression_statement()),
            };

            let cond = match self.peek().token_type {
                TokenType::SEMICOLON => None,
                _ => Some(self.expression()?),
            };
            // consume semicolon
            self.consume(&TokenType::SEMICOLON, "Expect ';' after for condition.")?;

            let inc = match self.peek().token_type {
                TokenType::RPAREN => None,
                _ => Some(self.expression()?),
            };

            self.consume(&TokenType::RPAREN, "Expect ')' after for clauses.")?;

            let body = self.statement()?;

            // append body and inc
            let body_inc: Stmt;
            if let Some(inc) = inc {
                body_inc = Stmt::Block(Box::new(Block {
                    statements: vec![
                        body,
                        Stmt::Expression(Box::new(Expression {
                            expression: Box::new(inc),
                        })),
                    ],
                }));
            } else {
                body_inc = Stmt::Block(Box::new(Block {
                    statements: vec![body],
                }));
            }

            // generate while body
            let while_body: Stmt;
            if let Some(cond) = cond {
                while_body = Stmt::WhileStmt(Box::new(WhileStmt {
                    condition: Box::new(cond),
                    body: Box::new(body_inc),
                }));
            } else {
                while_body = Stmt::WhileStmt(Box::new(WhileStmt {
                    condition: Box::new(Expr::Literal(Literal {
                        value: Object::Boolean(true),
                    })),
                    body: Box::new(body_inc),
                }));
            }

            // combine initializer and while
            if let Some(initializer) = initializer {
                Ok(Stmt::Block(Box::new(Block {
                    statements: vec![initializer?, while_body],
                })))
            } else {
                Ok(while_body)
            }
        } else {
            Err(LoxError::new_parse(
                self.peek().clone(),
                "Expect '(' after 'for'.",
            ))
        }
    }

    pub fn while_statement(&mut self) -> Result<Stmt> {
        if self.match_token(&[TokenType::LPAREN]) {
            let cond = self.expression()?;
            self.consume(&TokenType::RPAREN, "Expect ')' after condition.")?;

            let body = self.statement()?;

            Ok(Stmt::WhileStmt(Box::new(WhileStmt {
                condition: Box::new(cond),
                body: Box::new(body),
            })))
        } else {
            Err(LoxError::new_parse(
                self.peek().clone(),
                "Expect '(' after 'while'.",
            ))
        }
    }

    pub fn if_statement(&mut self) -> Result<Stmt> {
        if self.match_token(&[TokenType::LPAREN]) {
            // After the LPAREN, parse the condition next
            let cond = self.expression()?;
            // We've parsed the condition, now check for closing parenthesis

            self.consume(&TokenType::RPAREN, "Expect ')' after if condition.")?;
            // Right paren already consumed by consume above

            let body = self.statement()?;

            // check for else condition
            let else_branch = if self.match_token(&[TokenType::ELSE]) {
                Some(self.statement()?)
            } else {
                None
            };

            Ok(Stmt::IfStmt(Box::new(IfStmt {
                condition: Box::new(cond),
                then_branch: Box::new(body),
                else_branch: else_branch.map(|stmt| Box::new(stmt)),
            })))
        } else {
            return Err(LoxError::new_parse(
                self.peek().clone(),
                "Expect '(' after 'if'.",
            ));
        }
    }

    pub fn block(&mut self) -> Result<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RBRACE) && !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }

        self.consume(&TokenType::RBRACE, "Expect '}' after block.")?;
        Ok(Stmt::Block(Box::new(Block { statements: stmts })))
    }

    pub fn declaration(&mut self) -> Result<Stmt> {
        if self.match_token(&[TokenType::VAR]) {
            return self.var_declar();
        }
        self.statement()
    }

    pub fn var_declar(&mut self) -> Result<Stmt> {
        // 'var' already consumed by match_token in declaration()

        self.consume(&TokenType::IDENTIFIER, "Expect variable name.")?;
        let name = self.previous().clone();

        let initializer = if self.match_token(&[TokenType::EQUAL]) {
            // "=" already consumed by match_token
            let expr = self.expression()?;
            Box::new(expr)
        } else {
            Box::new(Expr::Literal(Literal { value: Object::Nil }))
        };

        self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        // Semicolon already consumed by match_token above
        Ok(Stmt::Var(Box::new(Var {
            name: Box::new(name),
            initializer,
        })))
    }

    pub fn statement(&mut self) -> Result<Stmt> {
        if self.match_token(&[TokenType::PRINT]) {
            // Print token already consumed by match_token
            return self.print_expression();
        } else if self.match_token(&[TokenType::LBRACE]) {
            return self.block();
        } else if self.match_token(&[TokenType::WHILE]) {
            return self.while_statement();
        } else if self.match_token(&[TokenType::FUN]) {
            return self.fun_statement();
        } else if self.match_token(&[TokenType::FOR]) {
            return self.for_statement();
        } else if self.match_token(&[TokenType::IF]) {
            return self.if_statement();
        } else {
            self.expression_statement()
        }
    }

    pub fn print_expression(&mut self) -> Result<Stmt> {
        let expr: Expr = self.expression()?;

        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;

        // Semicolon already consumed by consume above
        Ok(Stmt::Print(Box::new(Print {
            expression: Box::new(expr),
        })))
    }
    pub fn expression_statement(&mut self) -> Result<Stmt> {
        let expr: Expr = self.expression()?;

        self.consume(&TokenType::SEMICOLON, "Expect ';' after expression.")?;

        // Semicolon already consumed by consume above
        Ok(Stmt::Expression(Box::new(Expression {
            expression: Box::new(expr),
        })))
    }

    pub fn logic_or(&mut self) -> Result<Expr> {
        let mut expr = self.logic_and()?;

        while self.match_token(&[TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            expr = Expr::Logical(Box::new(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expr)
    }

    pub fn logic_and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expr)
    }

    pub fn equality(&mut self) -> Result<Expr> {
        // Implementation for equality parsing will go here
        let mut expr: Expr = self.ternary()?;
        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.previous().clone();
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

    pub fn ternary(&mut self) -> Result<Expr> {
        // ternary -> comparison ( ("?") expression (":") ternary)*;
        let mut expr: Expr = self.comparison()?;
        while self.match_token(&[TokenType::QUEST]) {
            let left = self.expression()?; // parse the left expression
            // check for ':' token
            self.consume(&TokenType::COLON, "Expect ':' after '?'")?;
            // Colon already consumed by consume above
            let right = self.ternary()?; // parse the right expression
            expr = Expr::Ternary(Box::new(Ternary {
                condition: Box::new(expr),
                true_branch: Box::new(left),
                false_branch: Box::new(right),
            }));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr> {
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
            let operator: Token = self.previous().clone(); // Get the operator token
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

    pub fn term(&mut self) -> Result<Expr> {
        // term -> factor ( ( "-" | "+" ) factor )*;

        let mut expr: Expr = self.factor()?;
        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous().clone(); // Get the operator token
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

    pub fn factor(&mut self) -> Result<Expr> {
        // factor -> unary ( ( "/" | "*" ) unary )*;

        let mut expr: Expr = self.unary()?;
        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous().clone(); // Get the operator token
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

    pub fn unary(&mut self) -> Result<Expr> {
        // unary -> ( "!" | "-" ) unary | call;

        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone(); // Get the operator token
            let right = self.unary()?; // Get the next unary expression

            // Create a Unary expression node wrapped in Expr enum
            return Ok(Expr::Unary(Box::new(Unary {
                operator,
                right: Box::new(right), // Box the right expression
            })));
        }

        // If not a unary operator, parse as primary
        self.call()
    }

    pub fn call(&mut self) -> Result<Expr> {
        // call -> primary ( "(" arguments ")" )?
        let mut callee = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LPAREN]) {
                callee = self.finish_call(callee)?;
            } else {
                break;
            }
        }

        Ok(callee)
    }

    pub fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = Vec::new();

        if !self.check(&TokenType::RPAREN) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RPAREN, "Expect ')' after arguments.")?;

        // Use the Call variant we defined in expr.rs
        Ok(Expr::Call(Box::new(Call {
            callee: Box::new(callee),
            paren: self.previous().clone(),
            args,
        })))
    }

    pub fn primary(&mut self) -> Result<Expr> {
        // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | identifier;
        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let token: Token = self.previous().clone(); // Get the token
            // Create a Literal expression node wrapped in Expr enum
            match token.token_type {
                TokenType::NUMBER => {
                    let literal_clone = token.literal.clone();
                    let value: f64 = literal_clone
                        .ok_or_else(|| {
                            LoxError::new_parse(token.clone(), "Missing literal value for number")
                        })?
                        .parse()
                        .map_err(|_| LoxError::new_parse(token.clone(), "Invalid number format"))?;

                    return Ok(Expr::Literal(Literal {
                        value: Object::Number(value), // Wrap in Object::Number
                    }));
                }
                TokenType::STRING => {
                    let literal_clone = token.literal.clone();
                    let value: String = literal_clone.ok_or_else(|| {
                        LoxError::new_parse(token.clone(), "Missing literal value for string")
                    })?;

                    return Ok(Expr::Literal(Literal {
                        value: Object::String(value), // Wrap in Object::String
                    }));
                }
                _ => {
                    // This should not happen as we already checked for NUMBER and STRING
                    return Err(LoxError::new_parse(
                        token,
                        "Unexpected token type in primary",
                    ));
                }
            }
        }

        if self.match_token(&[TokenType::TRUE]) {
            // Token already consumed by match_token
            return Ok(Expr::Literal(Literal {
                value: Object::Boolean(true), // Wrap in Object::Bool
            }));
        }

        if self.match_token(&[TokenType::FALSE]) {
            // Token already consumed by match_token
            return Ok(Expr::Literal(Literal {
                value: Object::Boolean(false), // Wrap in Object::Bool
            }));
        }

        if self.match_token(&[TokenType::NIL]) {
            // Token already consumed by match_token
            return Ok(Expr::Literal(Literal {
                value: Object::Nil, // Wrap in Object::Nil
            }));
        }

        if self.match_token(&[TokenType::LPAREN]) {
            let expr = self.expression()?; // Parse the inner expression

            self.consume(&TokenType::RPAREN, "Expected closing parenthesis")?;
            // Right paren already consumed by consume above

            return Ok(Expr::Grouping(Box::new(Grouping {
                expression: Box::new(expr),
            }))); // Grouping expression
        }

        if self.match_token(&[TokenType::IDENTIFIER]) {
            let expr_token = self.previous().clone();
            return Ok(Expr::Variable(Box::new(Variable { name: expr_token })));
        }

        // If none of the above, it's an error
        Err(LoxError::new_parse(
            self.peek().clone(),
            "Expect expression",
        ))
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    /// Checks if the current token matches any of the given types.
    /// If it does, consumes the token and returns true. Otherwise, returns false.
    ///
    /// This method both tests AND consumes the token if there's a match,
    /// so there's no need to call advance() after a successful match.
    pub fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Returns the most recently consumed token.
    /// Useful for accessing tokens after they've been matched and consumed.
    pub fn previous(&self) -> &Token {
        // Return the previous token (the one we just consumed)
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *token_type;
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
            || (self.current < self.tokens.len()
                && self.tokens[self.current].token_type == TokenType::EOF)
    }
    pub fn peek(&self) -> &Token {
        if self.is_at_end() {
            // check if last token is EOF - if so return it else error
            if self
                .tokens
                .last()
                .map_or(false, |t| t.token_type == TokenType::EOF)
            {
                return &self.tokens[self.tokens.len() - 1];
            }
            // Use the last token we have, even if it's not EOF
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[self.current]
    }

    pub fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::new_parse(self.peek().clone(), message))
        }
    }
}
