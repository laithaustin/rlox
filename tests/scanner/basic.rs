use super::*;

#[test]
fn test_empty_input() {
    let (tokens, reporter) = scan("");
    assert_eq!(tokens.len(), 1); // Only EOF token
    assert_token(&tokens[0], TokenType::EOF, "", 1);
    reporter.assert_no_errors();
}

#[test]
fn test_whitespace_only() {
    let (tokens, reporter) = scan("   \t\n\r");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_no_errors();
}

#[test]
fn test_single_character_tokens() {
    let (tokens, reporter) = scan("(){}");
    assert_token_sequence(&tokens, &[
        TokenType::LPAREN,
        TokenType::RPAREN,
        TokenType::LBRACE,
        TokenType::RBRACE,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_operators() {
    let (tokens, reporter) = scan("+ - * /");
    assert_token_sequence(&tokens, &[
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::STAR,
        TokenType::SLASH,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_double_character_operators() {
    let (tokens, reporter) = scan("!= == <= >=");
    assert_token_sequence(&tokens, &[
        TokenType::BANG_EQUAL,
        TokenType::EQUAL_EQUAL,
        TokenType::LESS_EQUAL,
        TokenType::GREATER_EQUAL,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_mixed_operators() {
    let (tokens, reporter) = scan("! = == < <= > >=");
    assert_token_sequence(&tokens, &[
        TokenType::BANG,
        TokenType::EQUAL,
        TokenType::EQUAL_EQUAL,
        TokenType::LESS,
        TokenType::LESS_EQUAL,
        TokenType::GREATER,
        TokenType::GREATER_EQUAL,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_line_tracking() {
    let (tokens, reporter) = scan("(\n)\n{\n}");
    assert_eq!(tokens.len(), 5); // 4 tokens + EOF
    assert_token(&tokens[0], TokenType::LPAREN, "(", 1);
    assert_token(&tokens[1], TokenType::RPAREN, ")", 2);
    assert_token(&tokens[2], TokenType::LBRACE, "{", 3);
    assert_token(&tokens[3], TokenType::RBRACE, "}", 4);
    reporter.assert_no_errors();
}

#[test]
fn test_whitespace_handling() {
    let (tokens, reporter) = scan("(  )\t{\t}\n,\n.\n;");
    assert_token_sequence(&tokens, &[
        TokenType::LPAREN,
        TokenType::RPAREN,
        TokenType::LBRACE,
        TokenType::RBRACE,
        TokenType::COMMA,
        TokenType::DOT,
        TokenType::SEMICOLON,
    ]);
    reporter.assert_no_errors();
} 