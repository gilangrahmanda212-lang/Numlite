use std::collections::HashMap;

use crate::ast::{BinOp, Expr};

pub type Env = HashMap<String, f64>;

pub fn eval(expr: &Expr, env: &Env) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Var(name) => env
            .get(name)
            .copied()
            .ok_or_else(|| format!("Variabel '{}' belum didefinisikan", name)),
        Expr::Binary { op, left, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err("Pembagian dengan nol".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}
