use super::*;

#[test]
fn test_unterminated_string() {
    let (tokens, reporter) = scan("\"hello");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unterminated string.")]);
}

#[test]
fn test_invalid_characters() {
    let (tokens, reporter) = scan("@#$%");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[
        (1, "Unexpected character '@'"),
        (1, "Unexpected character '#'"),
        (1, "Unexpected character '$'"),
        (1, "Unexpected character '%'"),
    ]);
}

#[test]
fn test_invalid_number() {
    let (tokens, reporter) = scan("123.456.789");
    assert_eq!(tokens.len(), 4); // NUMBER, DOT, NUMBER, EOF tokens
    assert_eq!(tokens[0].token_type, TokenType::NUMBER);
    assert_eq!(tokens[1].token_type, TokenType::DOT);
    assert_eq!(tokens[2].token_type, TokenType::NUMBER);
    reporter.assert_no_errors();
} 