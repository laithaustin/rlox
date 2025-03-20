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
        (1, "Unexpected character."),
        (1, "Unexpected character."),
        (1, "Unexpected character."),
        (1, "Unexpected character."),
    ]);
}

#[test]
fn test_invalid_number() {
    let (tokens, reporter) = scan("123.456.789");
    assert_eq!(tokens.len(), 1); // Only EOF token
    reporter.assert_errors(&[(1, "Unexpected character.")]);
} 