use std::fmt;
use std::str::FromStr;

// Establish basic token types for the lexer
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // single char tokens
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    QUEST,
    COLON,

    // one or two char tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl FromStr for TokenType {
    type Err = ();

    // TODO: apparently we can use strum macro to derive FromStr for enums withouth having to write
    // this boilerplate code
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Self::LPAREN),
            ")" => Ok(Self::RPAREN),
            "{" => Ok(Self::LBRACE),
            "}" => Ok(Self::RBRACE),
            "," => Ok(Self::COMMA),
            "." => Ok(Self::DOT),
            "-" => Ok(Self::MINUS),
            "+" => Ok(Self::PLUS),
            ";" => Ok(Self::SEMICOLON),
            "/" => Ok(Self::SLASH),
            "*" => Ok(Self::STAR),
            "!" => Ok(Self::BANG),
            "?" => Ok(Self::QUEST),
            ":" => Ok(Self::COLON),
            "!=" => Ok(Self::BANG_EQUAL),
            "=" => Ok(Self::EQUAL),
            "==" => Ok(Self::EQUAL_EQUAL),
            ">" => Ok(Self::GREATER),
            ">=" => Ok(Self::GREATER_EQUAL),
            "<" => Ok(Self::LESS),
            "<=" => Ok(Self::LESS_EQUAL),
            // keywords
            "and" => Ok(Self::AND),
            "class" => Ok(Self::CLASS),
            "else" => Ok(Self::ELSE),
            "false" => Ok(Self::FALSE),
            "fun" => Ok(Self::FUN),
            "for" => Ok(Self::FOR),
            "if" => Ok(Self::IF),
            "nil" => Ok(Self::NIL),
            "or" => Ok(Self::OR),
            "print" => Ok(Self::PRINT),
            "return" => Ok(Self::RETURN),
            "super" => Ok(Self::SUPER),
            "this" => Ok(Self::THIS),
            "true" => Ok(Self::TRUE),
            "var" => Ok(Self::VAR),
            "while" => Ok(Self::WHILE),
            "EOF" => Ok(Self::EOF),
            _ => Err(()),
        }
    }
}

// Token Struct
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<String>,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"".to_string())
        )
    }
}

// RuntimeError has been moved to the error.rs module
