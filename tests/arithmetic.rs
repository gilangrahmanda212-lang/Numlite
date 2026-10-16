use numlite::ast::Stmt;
use numlite::eval::{eval, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;
use numlite::value::Value;

fn calc(input: &str) -> Result<Value, String> {
    let env = Env::new();
    calc_with_env(input, &env)
}

fn calc_with_env(input: &str, env: &Env) -> Result<Value, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt()?;
    match stmt {
        Stmt::Expr(e) => eval(&e, env),
        Stmt::Assign { .. } => Err("bukan expression".to_string()),
    }
}

fn calc_scalar(input: &str) -> Result<f64, String> {
    calc(input)?.as_scalar().ok_or_else(|| "bukan scalar".to_string())
}

fn calc_scalar_with_env(input: &str, env: &Env) -> Result<f64, String> {
    calc_with_env(input, env)?
        .as_scalar()
        .ok_or_else(|| "bukan scalar".to_string())
}

#[test]
fn test_add() {
    assert_eq!(calc_scalar("1 + 2").unwrap(), 3.0);
}

#[test]
fn test_precedence() {
    assert_eq!(calc_scalar("1 + 2 * 3").unwrap(), 7.0);
    assert_eq!(calc_scalar("2 * 3 + 1").unwrap(), 7.0);
}

#[test]
fn test_parentheses() {
    assert_eq!(calc_scalar("(1 + 2) * 3").unwrap(), 9.0);
}

#[test]
fn test_left_associative() {
    assert_eq!(calc_scalar("10 - 2 - 3").unwrap(), 5.0);
    assert_eq!(calc_scalar("100 / 4 / 5").unwrap(), 5.0);
}

#[test]
fn test_float() {
    assert_eq!(calc_scalar("3.14 * 2").unwrap(), 6.28);
}

#[test]
fn test_divide_by_zero() {
    assert!(calc_scalar("1 / 0").is_err());
}

#[test]
fn test_syntax_error() {
    assert!(calc_scalar("1 +").is_err());
    assert!(calc_scalar("(1 + 2").is_err());
}

#[test]
fn test_variable() {
    let mut env = Env::new();
    env.insert("x".to_string(), Value::Scalar(5.0));
    env.insert("y".to_string(), Value::Scalar(3.0));
    assert_eq!(calc_scalar_with_env("x + y", &env).unwrap(), 8.0);
    assert_eq!(calc_scalar_with_env("x * 2", &env).unwrap(), 10.0);
}

#[test]
fn test_undefined_variable() {
    let env = Env::new();
    assert!(calc_scalar_with_env("z + 1", &env).is_err());
}
