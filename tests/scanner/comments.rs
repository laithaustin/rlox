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

// Tests for multiline comments
#[test]
fn test_basic_multiline_comment() {
    let (tokens, reporter) = scan("/* This is a multiline comment */\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_multiline_comment_with_newlines() {
    let (tokens, reporter) = scan("/* This is a \nmultiline\ncomment */\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_nested_multiline_comments() {
    let (tokens, reporter) = scan("/* Outer /* Nested */ Comment */\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_deeply_nested_multiline_comments() {
    let (tokens, reporter) = scan("/* Level 1 /* Level 2 /* Level 3 */ Level 2 */ Level 1 */\n123");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    assert_eq!(tokens[0].lexeme, "123");
    reporter.assert_no_errors();
}

#[test]
fn test_unterminated_multiline_comment() {
    let (tokens, reporter) = scan("/* This comment never ends");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unterminated multiline comment")]);
}

#[test]
fn test_unterminated_nested_multiline_comment() {
    let (tokens, reporter) = scan("/* Outer /* Nested */ still open");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unterminated multiline comment")]);
}

#[test]
fn test_code_with_multiline_comments() {
    let (tokens, reporter) = scan("123 /* comment */ + /* another */ 456");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER, 
        TokenType::PLUS, 
        TokenType::NUMBER
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_multiline_comment_at_eof() {
    let (tokens, reporter) = scan("123 /* comment */");
    assert_token_sequence(&tokens, &[TokenType::NUMBER]);
    reporter.assert_no_errors();
}