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
    }
}

fn apply_binop(op: BinOp, l: &Value, r: &Value) -> Result<Value, String> {
    // Kalau keduanya scalar → scalar arithmetic
    if let (Some(a), Some(b)) = (l.as_scalar(), r.as_scalar()) {
        return match op {
            BinOp::Add => Ok(Value::Scalar(a + b)),
            BinOp::Sub => Ok(Value::Scalar(a - b)),
            BinOp::Mul => Ok(Value::Scalar(a * b)),
            BinOp::Div => {
                if b == 0.0 {
                    Err("Pembagian dengan nol".to_string())
                } else {
                    Ok(Value::Scalar(a / b))
                }
            }
        };
    }

    // Kalau salah satu matriks → operasi element-wise
    let lm = l.as_matrix();
    let rm = r.as_matrix();

    // Untuk sekarang, hanya dukung dimensi yang sama (belum broadcast)
    if lm.rows != rm.rows || lm.cols != rm.cols {
        return Err(format!(
            "Dimensi tidak cocok: {}x{} vs {}x{}",
            lm.rows, lm.cols, rm.rows, rm.cols
        ));
    }

    let mut result = Matrix::zeros(lm.rows, lm.cols);
    for i in 0..lm.data.len() {
        let a = lm.data[i];
        let b = rm.data[i];
        result.data[i] = match op {
            BinOp::Add => a + b,
            BinOp::Sub => a - b,
            BinOp::Mul => a * b,
            BinOp::Div => {
                if b == 0.0 {
                    return Err("Pembagian dengan nol (elemen matriks)".to_string());
                }
                a / b
            }
        };
    }
    Ok(Value::Matrix(result))
}
