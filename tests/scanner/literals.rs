use super::*;

#[test]
fn test_integer_literals() {
    let (tokens, reporter) = scan("123 456 789");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::NUMBER,
        TokenType::NUMBER,
    ]);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[1].lexeme, "456");
    assert_eq!(tokens[2].lexeme, "789");
    reporter.assert_no_errors();
}

#[test]
fn test_decimal_literals() {
    let (tokens, reporter) = scan("123.456 789.012");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::NUMBER,
    ]);
    assert_eq!(tokens[0].lexeme, "123.456");
    assert_eq!(tokens[1].lexeme, "789.012");
    reporter.assert_no_errors();
}

#[test]
fn test_string_literals() {
    let (tokens, reporter) = scan("\"hello\" \"world\"");
    assert_token_sequence(&tokens, &[
        TokenType::STRING,
        TokenType::STRING,
    ]);
    assert_eq!(tokens[0].lexeme, "\"hello\"");
    assert_eq!(tokens[1].lexeme, "\"world\"");
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
fn test_string_with_escapes() {
    let (tokens, reporter) = scan("\"hello\\nworld\"");
    assert_token_sequence(&tokens, &[TokenType::STRING]);
    assert_eq!(tokens[0].lexeme, "\"hello\\nworld\"");
    reporter.assert_no_errors();
}

#[test]
fn test_mixed_literals() {
    let (tokens, reporter) = scan("123 \"hello\" 456.789 \"world\"");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::STRING,
        TokenType::NUMBER,
        TokenType::STRING,
    ]);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[1].lexeme, "\"hello\"");
    assert_eq!(tokens[2].lexeme, "456.789");
    assert_eq!(tokens[3].lexeme, "\"world\"");
    reporter.assert_no_errors();
}

#[test]
fn test_literals_with_operators() {
    let (tokens, reporter) = scan("123 + \"hello\" * 456.789");
    assert_token_sequence(&tokens, &[
        TokenType::NUMBER,
        TokenType::PLUS,
        TokenType::STRING,
        TokenType::STAR,
        TokenType::NUMBER,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_multiple_lines() {
    let (tokens, reporter) = scan("print\n\"hello\"\n123");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[1].line, 2);
    assert_eq!(tokens[2].line, 3);
    reporter.assert_no_errors();
} 