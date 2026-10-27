//cat > tests/arithmetic.rs << 'ENDOFFILE'
use numlite::ast::Stmt;
use numlite::eval::{eval, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;
use numlite::value::{Matrix, Value};

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

fn calc_matrix(input: &str) -> Result<Matrix, String> {
    match calc(input)? {
        Value::Matrix(m) => Ok(m),
        Value::Scalar(n) => Ok(Matrix::new(1, 1, vec![n])),
    }
}

#[test]
fn test_add() {
    assert_eq!(calc_scalar("1 + 2").unwrap(), 3.0);
}

#[test]
fn test_precedence() {
    assert_eq!(calc_scalar("1 + 2 * 3").unwrap(), 7.0);
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
}

#[test]
fn test_undefined_variable() {
    let env = Env::new();
    assert!(calc_scalar_with_env("z + 1", &env).is_err());
}

#[test]
fn test_matrix_literal_row() {
    let m = calc_matrix("[1, 2, 3]").unwrap();
    assert_eq!(m.rows, 1);
    assert_eq!(m.cols, 3);
    assert_eq!(m.data, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_matrix_literal_2x2() {
    let m = calc_matrix("[1, 2; 3, 4]").unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 2);
    assert_eq!(m.data, vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_matrix_literal_col() {
    let m = calc_matrix("[1; 2; 3]").unwrap();
    assert_eq!(m.rows, 3);
    assert_eq!(m.cols, 1);
}

#[test]
fn test_matrix_add() {
    let m = calc_matrix("[1, 2; 3, 4] + [10, 20; 30, 40]").unwrap();
    assert_eq!(m.data, vec![11.0, 22.0, 33.0, 44.0]);
}

#[test]
fn test_matrix_mul() {
    let m = calc_matrix("[1, 2; 3, 4] * [5, 6; 7, 8]").unwrap();
    assert_eq!(m.data, vec![19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn test_matrix_emul() {
    let m = calc_matrix("[1, 2; 3, 4] .* [5, 6; 7, 8]").unwrap();
    assert_eq!(m.data, vec![5.0, 12.0, 21.0, 32.0]);
}

#[test]
fn test_transpose() {
    let m = calc_matrix("[1, 2; 3, 4]'").unwrap();
    assert_eq!(m.data, vec![1.0, 3.0, 2.0, 4.0]);
}

#[test]
fn test_scalar_matrix_mul() {
    let m = calc_matrix("2 * [1, 2; 3, 4]").unwrap();
    assert_eq!(m.data, vec![2.0, 4.0, 6.0, 8.0]);
}

#[test]
fn test_zeros() {
    let m = calc_matrix("zeros(2, 3)").unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 3);
}

#[test]
fn test_eye() {
    let m = calc_matrix("eye(3)").unwrap();
    assert_eq!(m.data, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_size() {
    let m = calc_matrix("size([1, 2, 3; 4, 5, 6])").unwrap();
    assert_eq!(m.data, vec![2.0, 3.0]);
}

#[test]
fn test_sqrt() {
    assert_eq!(calc_scalar("sqrt(16)").unwrap(), 4.0);
}

#[test]
fn test_negative() {
    assert_eq!(calc_scalar("-5 + 10").unwrap(), 5.0);
}

//ENDOFFILE
