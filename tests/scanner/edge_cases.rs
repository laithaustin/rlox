use super::*;

#[test]
fn test_empty_input() {
    let (tokens, reporter) = scan("");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_no_errors();
}

#[test]
fn test_whitespace_only() {
    let (tokens, reporter) = scan("   \t\n\r  ");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_no_errors();
}

#[test]
fn test_unterminated_string() {
    let (tokens, reporter) = scan("\"unterminated");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unterminated string")]);
}

#[test]
fn test_invalid_character() {
    let (tokens, reporter) = scan("@");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unexpected character '@'")]);
}

#[test]
fn test_multiple_errors() {
    let (tokens, reporter) = scan("@ \"unterminated\n#");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[
        (1, "Unexpected character '@'"),
        (1, "Unterminated string"),
        (2, "Unexpected character '#'"),
    ]);
}

#[test]
fn test_mixed_valid_and_invalid() {
    let (tokens, reporter) = scan("123 @ \"hello\" # \"unterminated");
    assert_token_sequence(&tokens, &[TokenType::NUMBER, TokenType::STRING]);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[1].lexeme, "hello");
    reporter.assert_errors(&[
        (1, "Unexpected character '@'"),
        (1, "Unexpected character '#'"),
        (1, "Unterminated string"),
    ]);
}

#[test]
fn test_string_with_escape_sequences() {
    let (tokens, reporter) = scan(r#""Hello\n\t\"World\"""#);
    assert_token_sequence(&tokens, &[TokenType::STRING]);
    assert_eq!(tokens[0].lexeme, "Hello\n\t\"World\"");
    reporter.assert_no_errors();
}

#[test]
fn test_numbers_with_decimal() {
    let (tokens, reporter) = scan("123.456 .123 123.");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::DOT,
        TokenType::NUMBER,
        TokenType::NUMBER,
        TokenType::DOT,
    ]);
    assert_eq!(tokens[0].lexeme, "123.456");
    assert_eq!(tokens[2].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_identifiers_with_numbers() {
    let (tokens, reporter) = scan("abc123 123abc a1b2c3");
    assert_token_sequence(&tokens, &[
        TokenType::IDENTIFIER,
        TokenType::NUMBER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
    ]);
    assert_eq!(tokens[0].lexeme, "abc123");
    assert_eq!(tokens[1].lexeme, "123");
    assert_eq!(tokens[2].lexeme, "abc");
    assert_eq!(tokens[3].lexeme, "a1b2c3");
    reporter.assert_no_errors();
}

#[test]
fn test_empty_string() {
    let (tokens, reporter) = scan("\"\"");
    assert_token_sequence(&tokens, &[TokenType::STRING]);
    assert_eq!(tokens[0].lexeme, "\"\"");
    reporter.assert_no_errors();
}

#[test]
fn test_unicode_characters() {
    let (tokens, reporter) = scan("\"Hello, 世界!\"");
    assert_token_sequence(&tokens, &[TokenType::STRING]);
    assert_eq!(tokens[0].lexeme, "\"Hello, 世界!\"");
    reporter.assert_no_errors();
}

#[test]
fn test_very_large_number() {
    let (tokens, reporter) = scan("1234567890.1234567890");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "1234567890.1234567890");
    reporter.assert_no_errors();
}

#[test]
fn test_operators_without_spaces() {
    let (tokens, reporter) = scan("++--**//");
    assert_token_sequence(&tokens, &[
        TokenType::PLUS,
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::MINUS,
        TokenType::STAR,
        TokenType::STAR,
        TokenType::SLASH,
        TokenType::SLASH,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_mixed_operators_and_literals() {
    let (tokens, reporter) = scan("123+456-789*0/1");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::PLUS,
        TokenType::NUMBER,
        TokenType::MINUS,
        TokenType::NUMBER,
        TokenType::STAR,
        TokenType::NUMBER,
        TokenType::SLASH,
        TokenType::NUMBER,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_complex_expression() {
    let (tokens, reporter) = scan("(123 + 456) * (789 - 0)");
    assert_token_sequence(&tokens, &[
        TokenType::LPAREN,
        TokenType::NUMBER,
        TokenType::PLUS,
        TokenType::NUMBER,
        TokenType::RPAREN,
        TokenType::STAR,
        TokenType::LPAREN,
        TokenType::NUMBER,
        TokenType::MINUS,
        TokenType::NUMBER,
        TokenType::RPAREN,
    ]);
    reporter.assert_no_errors();
} 