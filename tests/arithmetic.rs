use numlite::eval::eval;
use numlite::lexer::Lexer;
use numlite::parser::Parser;

fn calc(input: &str) -> Result<f64, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    eval(&ast)
}

#[test]
fn test_add() {
    assert_eq!(calc("1 + 2").unwrap(), 3.0);
}

#[test]
fn test_precedence() {
    assert_eq!(calc("1 + 2 * 3").unwrap(), 7.0);
    assert_eq!(calc("2 * 3 + 1").unwrap(), 7.0);
}

#[test]
fn test_parentheses() {
    assert_eq!(calc("(1 + 2) * 3").unwrap(), 9.0);
}

#[test]
fn test_left_associative() {
    assert_eq!(calc("10 - 2 - 3").unwrap(), 5.0);
    assert_eq!(calc("100 / 4 / 5").unwrap(), 5.0);
}

#[test]
fn test_float() {
    assert_eq!(calc("3.14 * 2").unwrap(), 6.28);
}

#[test]
fn test_divide_by_zero() {
    assert!(calc("1 / 0").is_err());
}

#[test]
fn test_syntax_error() {
    assert!(calc("1 +").is_err());
    assert!(calc("(1 + 2").is_err());
}
