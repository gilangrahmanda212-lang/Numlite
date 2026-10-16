use crate::ast::{BinOp, Expr};

pub fn eval(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Binary { op, left, right } => {
            let l = eval(left)?;
            let r = eval(right)?;
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
