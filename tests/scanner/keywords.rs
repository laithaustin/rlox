use super::*;

#[test]
fn test_keywords() {
    let (tokens, reporter) = scan("and class else false for fun if nil or print return super this true var while");
    assert_token_sequence(&tokens, &[
        TokenType::AND,
        TokenType::CLASS,
        TokenType::ELSE,
        TokenType::FALSE,
        TokenType::FOR,
        TokenType::FUN,
        TokenType::IF,
        TokenType::NIL,
        TokenType::OR,
        TokenType::PRINT,
        TokenType::RETURN,
        TokenType::SUPER,
        TokenType::THIS,
        TokenType::TRUE,
        TokenType::VAR,
        TokenType::WHILE,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_identifiers() {
    let (tokens, reporter) = scan("andy classroom elsewhere falsify forest funny iffier nilpotent orca printer returner superman thirst truthy variant whiley");
    assert_token_sequence(&tokens, &[
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
        TokenType::IDENTIFIER,
    ]);
    reporter.assert_no_errors();
}

#[test]
fn test_mixed_keywords_and_identifiers() {
    let (tokens, reporter) = scan("if foo while bar");
    assert_token_sequence(&tokens, &[
        TokenType::IF,
        TokenType::IDENTIFIER,
        TokenType::WHILE,
        TokenType::IDENTIFIER,
    ]);
    assert_eq!(tokens[1].lexeme, "foo");
    assert_eq!(tokens[3].lexeme, "bar");
    reporter.assert_no_errors();
}

#[test]
fn test_keywords_as_identifiers() {
    let (tokens, reporter) = scan("printx classy notelse");
    assert_eq!(tokens.len() - 1, 3);
    assert_eq!(tokens[0].token_type, TokenType::IDENTIFIER, "Expected IDENTIFIER for 'printx', got {:?}", tokens[0].token_type);
    assert_eq!(tokens[1].token_type, TokenType::IDENTIFIER, "Expected IDENTIFIER for 'classy', got {:?}", tokens[1].token_type);
    assert_eq!(tokens[2].token_type, TokenType::IDENTIFIER, "Expected IDENTIFIER for 'notelse', got {:?}", tokens[2].token_type);
    reporter.assert_no_errors();
}

#[test]
fn test_keyword_case_sensitivity() {
    let (tokens, reporter) = scan("AND or TRUE false");
    assert_token_sequence(&tokens, &[
        TokenType::IDENTIFIER,
        TokenType::OR,
        TokenType::IDENTIFIER,
        TokenType::FALSE,
    ]);
    assert_eq!(tokens[0].lexeme, "AND");
    assert_eq!(tokens[2].lexeme, "TRUE");
    reporter.assert_no_errors();
}
