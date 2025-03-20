use super::*;

#[test]
fn test_single_line_comments() {
    let (tokens, reporter) = scan("// This is a comment\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_multiple_single_line_comments() {
    let (tokens, reporter) = scan("// First comment\n// Second comment\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_comment_at_end_of_line() {
    let (tokens, reporter) = scan("123 // This is a comment");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_empty_comment() {
    let (tokens, reporter) = scan("//\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_comment_with_code() {
    let (tokens, reporter) = scan("123 + 456 // This is a comment");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::PLUS,
        TokenType::NUMBER,
    ]);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[2].lexeme, "456");
    reporter.assert_no_errors();
}

#[test]
fn test_multiline_comments() {
    let (tokens, reporter) = scan("/* This is a\nmultiline comment */\nprint");
    assert_token_sequence(&tokens, &[TokenType::PRINT]);
    reporter.assert_no_errors();
}

#[test]
fn test_nested_comments() {
    let (tokens, reporter) = scan("/* Outer /* Inner */ */\nprint");
    assert_token_sequence(&tokens, &[TokenType::PRINT]);
    reporter.assert_no_errors();
}

#[test]
fn test_unterminated_multiline_comment() {
    let (tokens, reporter) = scan("/* Unterminated\nmultiline comment");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unterminated multiline comment")]);
} 