use std::collections::HashMap;

use crate::ast::{BinOp, Expr};
use crate::value::{Matrix, Value};

pub type Env = HashMap<String, Value>;

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Scalar(*n)),
        Expr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Variabel '{}' belum didefinisikan", name)),
        Expr::Binary { op, left, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            apply_binop(*op, &l, &r)
        }
        Expr::MatrixLit { rows } => eval_matrix_lit(rows, env),
        Expr::Transpose(inner) => {
            let v = eval(inner, env)?;
            let m = v.as_matrix();
            Ok(Value::Matrix(transpose(&m)))
        }
        Expr::Call { name, args } => {
            let mut values = Vec::new();
            for arg in args {
                values.push(eval(arg, env)?);
            }
            call_builtin(name, &values)
        }
    }
}

fn eval_matrix_lit(rows: &[Vec<Expr>], env: &Env) -> Result<Value, String> {
    if rows.is_empty() {
        // Matriks kosong []
        return Ok(Value::Matrix(Matrix::zeros(0, 0)));
    }

    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut data = Vec::with_capacity(nrows * ncols);

    for row in rows {
        for cell in row {
            let v = eval(cell, env)?;
            let s = v.as_scalar().ok_or_else(|| {
                "Elemen matriks harus scalar (matriks bersarang belum didukung)".to_string()
            })?;
            data.push(s);
        }
    }

    Ok(Value::Matrix(Matrix::new(nrows, ncols, data)))
}

fn transpose(m: &Matrix) -> Matrix {
    let mut result = Matrix::zeros(m.cols, m.rows);
    for r in 0..m.rows {
        for c in 0..m.cols {
            result.set(c, r, m.get(r, c));
        }
    }
    result
}

fn apply_binop(op: BinOp, l: &Value, r: &Value) -> Result<Value, String> {
    match op {
        BinOp::Add => elementwise(l, r, |a, b| Ok(a + b)),
        BinOp::Sub => elementwise(l, r, |a, b| Ok(a - b)),
        BinOp::EMul => elementwise(l, r, |a, b| Ok(a * b)),
        BinOp::EDiv => elementwise(l, r, |a, b| {
            if b == 0.0 {
                Err("Pembagian dengan nol".to_string())
            } else {
                Ok(a / b)
            }
        }),
        BinOp::EPow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        BinOp::Mul => matrix_mul(l, r),
        BinOp::Div => {
            // Untuk sekarang, / pada scalar = pembagian biasa
            // / pada matriks = right division (nanti)
            elementwise(l, r, |a, b| {
                if b == 0.0 {
                    Err("Pembagian dengan nol".to_string())
                } else {
                    Ok(a / b)
                }
            })
        }
        BinOp::Pow => {
            // Untuk sekarang, ^ pada scalar = power biasa
            // ^ pada matriks = matrix power (nanti)
            elementwise(l, r, |a, b| Ok(a.powf(b)))
        }
    }
}

fn elementwise<F>(l: &Value, r: &Value, f: F) -> Result<Value, String>
where
    F: Fn(f64, f64) -> Result<f64, String>,
{
    // Fast path: scalar op scalar
    if let (Value::Scalar(a), Value::Scalar(b)) = (l, r) {
        return f(*a, *b).map(Value::Scalar);
    }

    // Kalau salah satu scalar, "broadcast" ke matriks lain
    if let (Value::Scalar(a), Value::Matrix(m)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() {
            *v = f(*a, *v)?;
        }
        return Ok(Value::Matrix(result));
    }
    if let (Value::Matrix(m), Value::Scalar(b)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() {
            *v = f(*v, *b)?;
        }
        return Ok(Value::Matrix(result));
    }

    // Matriks op matriks
    let (lm, rm) = (l.as_matrix(), r.as_matrix());
    if lm.rows != rm.rows || lm.cols != rm.cols {
        return Err(format!(
            "Dimensi tidak cocok: {}x{} vs {}x{}",
            lm.rows, lm.cols, rm.rows, rm.cols
        ));
    }

    let mut result = Matrix::zeros(lm.rows, lm.cols);
    for i in 0..lm.data.len() {
        result.data[i] = f(lm.data[i], rm.data[i])?;
    }
    Ok(Value::Matrix(result))
}

fn matrix_mul(l: &Value, r: &Value) -> Result<Value, String> {
    // Matrix-matrix multiplication
    if let (Value::Matrix(a), Value::Matrix(b)) = (l, r) {
        if a.cols != b.rows {
            return Err(format!(
                "Matrix multiplication tidak valid: {}x{} * {}x{}",
                a.rows, a.cols, b.rows, b.cols
            ));
        }
        let mut result = Matrix::zeros(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    sum += a.get(i, k) * b.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        return Ok(Value::Matrix(result));
    }

    // Scalar * scalar
    if let (Value::Scalar(a), Value::Scalar(b)) = (l, r) {
        return Ok(Value::Scalar(a * b));
    }

    // Scalar * Matrix atau Matrix * Scalar
    if let (Value::Scalar(s), Value::Matrix(m)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() {
            *v *= s;
        }
        return Ok(Value::Matrix(result));
    }
    if let (Value::Matrix(m), Value::Scalar(s)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() {
            *v *= s;
        }
        return Ok(Value::Matrix(result));
    }

    Err("Kombinasi tipe tidak valid untuk *".to_string())
}

fn call_builtin(name: &str, args: &[Value]) -> Result<Value, String> {
    match (name, args.len()) {
        ("zeros", 1) => {
            let n = args[0].as_scalar().ok_or("zeros: argumen harus scalar")? as usize;
            Ok(Value::Matrix(Matrix::zeros(n, n)))
        }
        ("zeros", 2) => {
            let r = args[0].as_scalar().ok_or("zeros: argumen harus scalar")? as usize;
            let c = args[1].as_scalar().ok_or("zeros: argumen harus scalar")? as usize;
            Ok(Value::Matrix(Matrix::zeros(r, c)))
        }
        ("ones", 1) => {
            let n = args[0].as_scalar().ok_or("ones: argumen harus scalar")? as usize;
            Ok(Value::Matrix(Matrix::ones(n, n)))
        }
        ("ones", 2) => {
            let r = args[0].as_scalar().ok_or("ones: argumen harus scalar")? as usize;
            let c = args[1].as_scalar().ok_or("ones: argumen harus scalar")? as usize;
            Ok(Value::Matrix(Matrix::ones(r, c)))
        }
        ("eye", 1) => {
            let n = args[0].as_scalar().ok_or("eye: argumen harus scalar")? as usize;
            Ok(Value::Matrix(Matrix::eye(n)))
        }
        ("length", 1) => {
            let m = args[0].as_matrix();
            Ok(Value::Scalar(m.data.len().max(m.rows).max(m.cols) as f64))
        }
        ("size", 1) => {
            let m = args[0].as_matrix();
            Ok(Value::Matrix(Matrix::new(1, 2, vec![m.rows as f64, m.cols as f64])))
        }
        ("sqrt", 1) => unary_fn("sqrt", &args[0], |x| x.sqrt()),
        ("abs", 1) => unary_fn("abs", &args[0], |x| x.abs()),
        ("sin", 1) => unary_fn("sin", &args[0], |x| x.sin()),
        ("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
        ("exp", 1) => unary_fn("exp", &args[0], |x| x.exp()),
        ("log", 1) => unary_fn("log", &args[0], |x| x.ln()),
        _ => Err(format!(
            "Fungsi '{}' dengan {} argumen belum didukung",
            name,
            args.len()
        )),
    }
}

fn unary_fn<F>(name: &str, arg: &Value, f: F) -> Result<Value, String>
where
    F: Fn(f64) -> f64,
{
    match arg {
        Value::Scalar(x) => Ok(Value::Scalar(f(*x))),
        Value::Matrix(m) => {
            let mut result = m.clone();
            for v in result.data.iter_mut() {
                *v = f(*v);
            }
            Ok(Value::Matrix(result))
        }
    }
    .map_err(|e: String| format!("{}: {}", name, e))
}
