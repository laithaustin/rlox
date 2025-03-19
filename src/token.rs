// Estalish basic token types for the lexer
#[derive(Debug)] // derive Debug trait for TokenType
pub enum TokenType {
    // single char tokens
    LPAREN, RPAREN, LBRACE, RBRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // one or two char tokens
    BANG, BANG_EQUAL, 
    EQUAL, EQUAL_EQUAL, 
    GREATER, GREATER_EQUAL, 
    LESS, LESS_EQUAL, 

    // literals
    IDENTIFIER, STRING, NUMBER,

    // keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

// Token Struct
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, literal: Option<String>) -> Self {
        Self { token_type, lexeme, line, literal }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal.as_ref().unwrap_or(&"".to_string()))
    }
}
