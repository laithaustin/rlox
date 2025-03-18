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
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<String>,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize, literal: Option<String>) -> Self {
        Self { token_type, lexeme, line, literal }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal.as_ref().unwrap_or(&"".to_string()))
    }
}
