use std::collections::HashMap;
use crate::ast::{BinOp, Expr, Stmt};
use crate::value::{Matrix, Value};

pub type Env = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
pub enum ControlFlow {
    Normal,
    Break,
    Continue,
}

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Scalar(*n)),
        Expr::String(s) => Ok(Value::Str(s.clone())),
        Expr::Var(name) => env.get(name).cloned().ok_or_else(|| format!("Variabel '{}' belum didefinisikan", name)),
        Expr::Not(inner) => {
            let v = eval(inner, env)?.as_scalar().unwrap_or(0.0);
            Ok(Value::Scalar(if v == 0.0 { 1.0 } else { 0.0 }))
        }
        Expr::Binary { op, left, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            apply_binop(*op, &l, &r)
        }
        Expr::MatrixLit { rows } => eval_matrix_lit(rows, env),
        Expr::Transpose(inner) => {
            let v = eval(inner, env)?;
            Ok(Value::Matrix(transpose(&v.as_matrix())))
        }
        Expr::Call { name, args } => {
            let mut values = Vec::new();
            for arg in args { values.push(eval(arg, env)?); }
            
            if let Some(Value::Function(f)) = env.get(name) {
                if f.params.len() != values.len() {
                    return Err(format!("Fungsi '{}' butuh {} argumen, diberi {}", name, f.params.len(), values.len()));
                }
                let mut local_env = env.clone();
                for (param, val) in f.params.iter().zip(values.iter()) {
                    local_env.insert(param.clone(), val.clone());
                }
                let mut captured = String::new();
                for stmt in &f.body {
                    exec_stmt(stmt, &mut local_env, &mut captured, false)?;
                }
                
                let mut return_val = None;
                for (key, val) in &local_env {
                    if !env.contains_key(key) && !f.params.contains(key) {
                        return_val = Some(val.clone());
                        break;
                    }
                }
                if return_val.is_none() {
                    return_val = local_env.get(name).or_else(|| local_env.get("ans")).cloned();
                }

                if let Some(v) = return_val { Ok(v) }
                else { Err(format!("Fungsi '{}' tidak mengembalikan nilai", name)) }
            } else {
                call_builtin(name, &values)
            }
        }
        Expr::Range { start, step, end } => {
            let start_val = eval(start, env)?.as_scalar().ok_or("Range: start harus scalar")?;
            let end_val = eval(end, env)?.as_scalar().ok_or("Range: end harus scalar")?;
            let step_val = if let Some(s) = step { eval(s, env)?.as_scalar().ok_or("Range: step harus scalar")? } else { 1.0 };
            if step_val == 0.0 { return Err("Range: step tidak boleh 0".to_string()); }
            let mut data = Vec::new();
            let mut current = start_val;
            if step_val > 0.0 {
                while current <= end_val + 1e-9 { data.push(current); current += step_val; }
            } else {
                while current >= end_val - 1e-9 { data.push(current); current += step_val; }
            }
            Ok(Value::Matrix(Matrix::new(1, data.len(), data)))
        }
        Expr::Index { target, index } => {
            let target_val = eval(target, env)?;
            let mat = target_val.as_matrix();
            let idx_mat = if let Expr::Call { name, .. } = &**index {
                if name == "get_end" {
                    Matrix::new(1, 1, vec![mat.data.len() as f64])
                } else { eval(index, env)?.as_matrix() }
            } else { eval(index, env)?.as_matrix() };

            let mut result_data = Vec::new();
            for &idx in &idx_mat.data {
                let i = (idx as usize).saturating_sub(1);
                if i < mat.data.len() { result_data.push(mat.data[i]); }
                else { return Err(format!("Index {} di luar batas array", idx)); }
            }
            Ok(Value::Matrix(Matrix::new(1, result_data.len(), result_data)))
        }
    }
}

fn eval_matrix_lit(rows: &[Vec<Expr>], env: &Env) -> Result<Value, String> {
    if rows.is_empty() { return Ok(Value::Matrix(Matrix::zeros(0, 0))); }
    let mut data = Vec::new();
    for row in rows {
        for cell in row {
            data.push(eval(cell, env)?.as_scalar().ok_or("Elemen harus scalar")?);
        }
    }
    Ok(Value::Matrix(Matrix::new(rows.len(), rows[0].len(), data)))
}

fn transpose(m: &Matrix) -> Matrix {
    let mut res = Matrix::zeros(m.cols, m.rows);
    for r in 0..m.rows {
        for c in 0..m.cols { res.set(c, r, m.get(r, c)); }
    }
    res
}

fn apply_binop(op: BinOp, l: &Value, r: &Value) -> Result<Value, String> {
    match op {
        BinOp::Add => elementwise(l, r, |a, b| Ok(a + b)),
        BinOp::Sub => elementwise(l, r, |a, b| Ok(a - b)),
        BinOp::EMul => elementwise(l, r, |a, b| Ok(a * b)),
        BinOp::EDiv => elementwise(l, r, |a, b| if b == 0.0 { Err("Div 0".into()) } else { Ok(a / b) }),
        BinOp::EPow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        BinOp::Mul => matrix_mul(l, r),
        BinOp::Div => elementwise(l, r, |a, b| if b == 0.0 { Err("Div 0".into()) } else { Ok(a / b) }),
        BinOp::Pow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        
        BinOp::Eq => elementwise(l, r, |a, b| Ok(if a == b { 1.0 } else { 0.0 })),
        BinOp::Neq => elementwise(l, r, |a, b| Ok(if a != b { 1.0 } else { 0.0 })),
        BinOp::Lt => elementwise(l, r, |a, b| Ok(if a < b { 1.0 } else { 0.0 })),
        BinOp::Gt => elementwise(l, r, |a, b| Ok(if a > b { 1.0 } else { 0.0 })),
        BinOp::Le => elementwise(l, r, |a, b| Ok(if a <= b { 1.0 } else { 0.0 })),
        BinOp::Ge => elementwise(l, r, |a, b| Ok(if a >= b { 1.0 } else { 0.0 })),
        
        BinOp::And => elementwise(l, r, |a, b| Ok(if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 })),
        BinOp::Or => elementwise(l, r, |a, b| Ok(if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 })),
    }
}

fn elementwise<F>(l: &Value, r: &Value, f: F) -> Result<Value, String>
where F: Fn(f64, f64) -> Result<f64, String> {
    if let (Value::Scalar(a), Value::Scalar(b)) = (l, r) { return f(*a, *b).map(Value::Scalar); }
    if let (Value::Scalar(a), Value::Matrix(m)) = (l, r) {
        let mut res = m.clone();
        for v in res.data.iter_mut() { *v = f(*a, *v)?; }
        return Ok(Value::Matrix(res));
    }
    if let (Value::Matrix(m), Value::Scalar(b)) = (l, r) {
        let mut res = m.clone();
        for v in res.data.iter_mut() { *v = f(*v, *b)?; }
        return Ok(Value::Matrix(res));
    }
    let (lm, rm) = (l.as_matrix(), r.as_matrix());
    if lm.rows != rm.rows || lm.cols != rm.cols { return Err("Dimensi tidak cocok".into()); }
    let mut res = Matrix::zeros(lm.rows, lm.cols);
    for i in 0..lm.data.len() { res.data[i] = f(lm.data[i], rm.data[i])?; }
    Ok(Value::Matrix(res))
}

fn matrix_mul(l: &Value, r: &Value) -> Result<Value, String> {
    if let (Value::Matrix(a), Value::Matrix(b)) = (l, r) {
        if a.cols != b.rows { return Err("Matrix mul tidak valid".into()); }
        let mut res = Matrix::zeros(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols { sum += a.get(i, k) * b.get(k, j); }
                res.set(i, j, sum);
            }
        }
        return Ok(Value::Matrix(res));
    }
    elementwise(l, r, |a, b| Ok(a * b))
}

pub fn exec_stmt(stmt: &Stmt, env: &mut Env, capture_output: &mut String, is_in_loop: bool) -> Result<ControlFlow, String> {
    match stmt {
        Stmt::Expr(Expr::Call { name, args }) if name == "run" => {
            if args.len() != 1 { return Err("run: butuh 1 argumen".to_string()); }
            let filename_val = eval(&args[0], env)?;
            let filename = match filename_val {
                Value::Str(s) => s,
                _ => return Err("run: nama file harus string".to_string()),
            };
            let content = std::fs::read_to_string(&filename)
                .map_err(|e| format!("Gagal membaca file {}: {}", filename, e))?;
            
            let mut lexer = crate::lexer::Lexer::new(&content);
            let tokens = lexer.tokenize()?;
            let mut parser = crate::parser::Parser::new(tokens);

            while !parser.is_at_eof() {
                if let Ok(stmt) = parser.parse_stmt() {
                    exec_stmt(&stmt, env, capture_output, is_in_loop)?;
                } else { break; }
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::Plot { .. } => Ok(ControlFlow::Normal),
        Stmt::Expr(Expr::Call { name, .. }) if name == "subplot" || name == "figure" || name == "grid" || name == "clc" || name == "clear" || name == "close" => {
            Ok(ControlFlow::Normal)
        }
        Stmt::Expr(expr) => {
            if let Expr::Call { name, args } = expr {
                if name == "disp" && args.len() == 1 {
                    let val = eval(&args[0], env)?;
                    match val {
                        Value::Str(s) => capture_output.push_str(&format!("{}\n", s)),
                        Value::Scalar(n) => capture_output.push_str(&format!("{}\n", n)),
                        v => capture_output.push_str(&format!("{}\n", v)),
                    }
                    return Ok(ControlFlow::Normal);
                }
            }

            let val = eval(expr, env)?;
            match &val {
                Value::Scalar(n) => capture_output.push_str(&format!("ans = {}\n", n)),
                Value::Complex(_) => capture_output.push_str(&format!("ans = {}\n", val)),
                _ => capture_output.push_str(&format!("ans =\n{}\n", val)),
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::Assign { name, value } => {
            let v = eval(value, env)?;
            if !is_in_loop {
                match &v {
                    Value::Scalar(n) => capture_output.push_str(&format!("{} = {}\n", name, n)),
                    Value::Complex(_) => capture_output.push_str(&format!("{} = {}\n", name, v)),
                    _ => capture_output.push_str(&format!("{} =\n{}\n", name, v)),
                }
            }
            env.insert(name.clone(), v);
            Ok(ControlFlow::Normal)
        }
        Stmt::For { var, iter, body } => {
            let iter_val = eval(iter, env)?;
            for val in iter_val.as_matrix().data {
                env.insert(var.clone(), Value::Scalar(val));
                let mut should_break = false;
                for s in body { 
                    match exec_stmt(s, env, capture_output, true)? {
                        ControlFlow::Break => { should_break = true; break; } 
                        ControlFlow::Continue => break, 
                        ControlFlow::Normal => {}
                    }
                }
                if should_break { break; }
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::While { cond, body } => {
            loop {
                let cond_val = eval(cond, env)?;
                let is_true = cond_val.as_scalar().unwrap_or(0.0) != 0.0;
                if !is_true { break; }
                let mut should_break = false;
                for s in body { 
                    match exec_stmt(s, env, capture_output, true)? {
                        ControlFlow::Break => { should_break = true; break; }
                        ControlFlow::Continue => { break; }
                        ControlFlow::Normal => {}
                    }
                }
                if should_break { break; }
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::If { cond, then_body, else_body } => {
            let cond_val = eval(cond, env)?;
            let is_true = cond_val.as_scalar().unwrap_or(0.0) != 0.0;
            if is_true {
                for s in then_body { 
                    let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                    if cf != ControlFlow::Normal { return Ok(cf); }
                }
            } else if let Some(eb) = else_body {
                for s in eb { 
                    let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                    if cf != ControlFlow::Normal { return Ok(cf); }
                }
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::Switch { expr, cases, otherwise } => {
            let val = eval(expr, env)?;
            let mut matched = false;
            for (case_expr, body) in cases {
                let case_val = eval(case_expr, env)?;
                let is_match = match (&val, &case_val) {
                    (Value::Scalar(a), Value::Scalar(b)) => (a - b).abs() < 1e-9,
                    (Value::Str(a), Value::Str(b)) => a == b,
                    _ => false,
                };
                if is_match {
                    for s in body {
                        let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                        if cf != ControlFlow::Normal { return Ok(cf); }
                    }
                    matched = true;
                    break;
                }
            }
            if !matched {
                if let Some(eb) = otherwise {
                    for s in eb {
                        let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                        if cf != ControlFlow::Normal { return Ok(cf); }
                    }
                }
            }
            Ok(ControlFlow::Normal)
        }
        Stmt::Break => Ok(ControlFlow::Break),
        Stmt::Continue => Ok(ControlFlow::Continue),
        Stmt::FunctionDef(f) => {
            env.insert(f.name.clone(), Value::Function(f.clone()));
            Ok(ControlFlow::Normal)
        }
    }
}

fn call_builtin(name: &str, args: &[Value]) -> Result<Value, String> {
    match (name, args.len()) {
        ("zeros", 1) => { let n = args[0].as_scalar().ok_or("zeros: argumen harus scalar")? as usize; Ok(Value::Matrix(Matrix::zeros(n, n))) }
        ("zeros", 2) => { let r = args[0].as_scalar().ok_or("zeros: argumen harus scalar")? as usize; let c = args[1].as_scalar().ok_or("zeros: argumen harus scalar")? as usize; Ok(Value::Matrix(Matrix::zeros(r, c))) }
        ("ones", 1) => { let n = args[0].as_scalar().ok_or("ones: argumen harus scalar")? as usize; Ok(Value::Matrix(Matrix::ones(n, n))) }
        ("ones", 2) => { let r = args[0].as_scalar().ok_or("ones: argumen harus scalar")? as usize; let c = args[1].as_scalar().ok_or("ones: argumen harus scalar")? as usize; Ok(Value::Matrix(Matrix::ones(r, c))) }
        ("eye", 1) => { let n = args[0].as_scalar().ok_or("eye: argumen harus scalar")? as usize; Ok(Value::Matrix(Matrix::eye(n))) }
        ("horzcat", 2) => {
            let a = args[0].as_matrix(); let b = args[1].as_matrix();
            if a.rows != b.rows { return Err("horzcat: jumlah baris harus sama".to_string()); }
            let new_cols = a.cols + b.cols; let mut data = Vec::with_capacity(a.rows * new_cols);
            for r in 0..a.rows {
                for c in 0..a.cols { data.push(a.get(r, c)); }
                for c in 0..b.cols { data.push(b.get(r, c)); }
            }
            Ok(Value::Matrix(Matrix::new(a.rows, new_cols, data)))
        }
        ("diag", 1) => {
            let m = args[0].as_matrix();
            if m.rows == 1 || m.cols == 1 {
                let n = m.data.len(); let mut res = Matrix::zeros(n, n);
                for i in 0..n { res.set(i, i, m.data[i]); }
                Ok(Value::Matrix(res))
            } else {
                let n = m.rows.min(m.cols); let mut data = Vec::with_capacity(n);
                for i in 0..n { data.push(m.get(i, i)); }
                Ok(Value::Matrix(Matrix::new(n, 1, data)))
            }
        }
        ("repmat", 3) => {
            let m = args[0].as_matrix();
            let rep_r = args[1].as_scalar().ok_or("repmat: baris harus scalar")? as usize;
            let rep_c = args[2].as_scalar().ok_or("repmat: kolom harus scalar")? as usize;
            let new_rows = m.rows * rep_r; let new_cols = m.cols * rep_c;
            let mut data = Vec::with_capacity(new_rows * new_cols);
            for _r in 0..rep_r { for r in 0..m.rows { for _c in 0..rep_c { for c in 0..m.cols { data.push(m.get(r, c)); } } } }
            Ok(Value::Matrix(Matrix::new(new_rows, new_cols, data)))
        }
        ("vertcat", 2) => {
            let a = args[0].as_matrix(); let b = args[1].as_matrix();
            if a.cols != b.cols { return Err("vertcat: jumlah kolom harus sama".to_string()); }
            let mut data = a.data.clone(); data.extend_from_slice(&b.data);
            Ok(Value::Matrix(Matrix::new(a.rows + b.rows, a.cols, data)))
        }
        ("reshape", 3) => {
            let m = args[0].as_matrix();
            let new_rows = args[1].as_scalar().ok_or("reshape: argumen baris harus scalar")? as usize;
            let new_cols = args[2].as_scalar().ok_or("reshape: argumen kolom harus scalar")? as usize;
            if new_rows * new_cols != m.data.len() { return Err(format!("reshape: dimensi tidak cocok")); }
            Ok(Value::Matrix(Matrix::new(new_rows, new_cols, m.data)))
        }
        ("length", 1) => { let m = args[0].as_matrix(); Ok(Value::Scalar(m.data.len().max(m.rows).max(m.cols) as f64)) }
        ("size", 1) => { let m = args[0].as_matrix(); Ok(Value::Matrix(Matrix::new(1, 2, vec![m.rows as f64, m.cols as f64]))) }
        ("inv", 1) => { let m = args[0].as_matrix(); let result = invert_matrix(&m)?; Ok(Value::Matrix(result)) }
        ("det", 1) => { let m = args[0].as_matrix(); if m.rows != m.cols { return Err("det: matriks harus persegi".to_string()); } Ok(Value::Scalar(determinant(&m)?)) }
        ("trace", 1) => { let m = args[0].as_matrix(); if m.rows != m.cols { return Err("trace: matriks harus persegi".to_string()); } let mut sum = 0.0; for i in 0..m.rows { sum += m.get(i, i); } Ok(Value::Scalar(sum)) }
        ("rank", 1) => { let m = args[0].as_matrix(); Ok(Value::Scalar(matrix_rank(&m))) }
        ("eig", 1) => { let m = args[0].as_matrix(); if m.rows != m.cols { return Err("eig: matriks harus persegi".to_string()); } Ok(Value::Scalar(power_iteration(&m, 100)?)) }

        ("round", 1) => unary_fn("round", &args[0], |x| x.round()),
        ("floor", 1) => unary_fn("floor", &args[0], |x| x.floor()),
        ("ceil", 1) => unary_fn("ceil", &args[0], |x| x.ceil()),
        ("mod", 2) => elementwise(&args[0], &args[1], |a, b| if b == 0.0 { Err("mod: div 0".into()) } else { Ok(a - b * (a / b).floor()) }),
        
        ("complex", 2) => { let re = args[0].as_scalar().ok_or("complex: butuh scalar")?; let im = args[1].as_scalar().ok_or("complex: butuh scalar")?; Ok(Value::Complex(crate::value::Complex::new(re, im))) }
        ("real", 1) => if let Some(c) = args[0].as_complex() { Ok(Value::Scalar(c.re)) } else { Err("real: butuh kompleks".to_string()) },
        ("imag", 1) => if let Some(c) = args[0].as_complex() { Ok(Value::Scalar(c.im)) } else { Err("imag: butuh kompleks".to_string()) },
        ("conj", 1) => if let Some(c) = args[0].as_complex() { Ok(Value::Complex(crate::value::Complex::new(c.re, -c.im))) } else { Err("conj: butuh kompleks".to_string()) },
        ("abs", 1) => if let Some(c) = args[0].as_complex() { Ok(Value::Scalar((c.re * c.re + c.im * c.im).sqrt())) } else { unary_fn("abs", &args[0], |x| x.abs()) },
        ("angle", 1) => if let Some(c) = args[0].as_complex() { Ok(Value::Scalar(c.im.atan2(c.re))) } else { Err("angle: butuh kompleks".to_string()) },

        ("sqrt", 1) => unary_fn("sqrt", &args[0], |x| x.sqrt()),
        ("sin", 1) => unary_fn("sin", &args[0], |x| x.sin()),
        ("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
        ("tan", 1) => unary_fn("tan", &args[0], |x| x.tan()),
        ("asin", 1) => unary_fn("asin", &args[0], |x| x.asin()),
        ("acos", 1) => unary_fn("acos", &args[0], |x| x.acos()),
        ("atan", 1) => unary_fn("atan", &args[0], |x| x.atan()),
        ("sign", 1) => unary_fn("sign", &args[0], |x| { if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { 0.0 } }),
        ("exp", 1) => unary_fn("exp", &args[0], |x| x.exp()),
        ("log", 1) => unary_fn("log", &args[0], |x| x.ln()),

        ("linspace", 3) => {
            let start = args[0].as_scalar().ok_or("linspace: butuh scalar")?;
            let end = args[1].as_scalar().ok_or("linspace: butuh scalar")?;
            let n = args[2].as_scalar().ok_or("linspace: butuh scalar")? as usize;
            if n < 2 { return Ok(Value::Matrix(Matrix::new(1, 1, vec![start]))); }
            let mut data = Vec::with_capacity(n); let step = (end - start) / (n as f64 - 1.0);
            for i in 0..n { data.push(start + step * i as f64); }
            Ok(Value::Matrix(Matrix::new(1, n, data)))
        }
        
        ("rand", 1) => { let n = args[0].as_scalar().ok_or("rand: butuh scalar")? as usize; let mut data = Vec::with_capacity(n * n); for _ in 0..(n * n) { data.push(rand::random::<f64>()); } Ok(Value::Matrix(Matrix::new(n, n, data))) }
        ("rand", 2) => { let r = args[0].as_scalar().ok_or("rand: butuh scalar")? as usize; let c = args[1].as_scalar().ok_or("rand: butuh scalar")? as usize; let mut data = Vec::with_capacity(r * c); for _ in 0..(r * c) { data.push(rand::random::<f64>()); } Ok(Value::Matrix(Matrix::new(r, c, data))) }
        ("randn", 1) => { let n = args[0].as_scalar().ok_or("randn: butuh scalar")? as usize; let mut data = Vec::with_capacity(n * n); for _ in 0..(n * n) { data.push(randn_scalar()); } Ok(Value::Matrix(Matrix::new(n, n, data))) }
        ("randn", 2) => { let r = args[0].as_scalar().ok_or("randn: butuh scalar")? as usize; let c = args[1].as_scalar().ok_or("randn: butuh scalar")? as usize; let mut data = Vec::with_capacity(r * c); for _ in 0..(r * c) { data.push(randn_scalar()); } Ok(Value::Matrix(Matrix::new(r, c, data))) }

        ("conv", 2) => {
            let a = args[0].as_matrix(); let b = args[1].as_matrix();
            if (a.rows > 1 && a.cols > 1) || (b.rows > 1 && b.cols > 1) { return Err("conv: input harus vektor 1D".to_string()); }
            let vec_a = &a.data; let vec_b = &b.data;
            let out_len = vec_a.len() + vec_b.len() - 1;
            let mut result = vec![0.0; out_len];
            for i in 0..vec_a.len() { for j in 0..vec_b.len() { result[i + j] += vec_a[i] * vec_b[j]; } }
            Ok(Value::Matrix(Matrix::new(1, out_len, result)))
        }

        ("mean", 1) => { let m = args[0].as_matrix(); if m.data.is_empty() { return Ok(Value::Scalar(0.0)); } let sum: f64 = m.data.iter().sum(); Ok(Value::Scalar(sum / m.data.len() as f64)) }
        ("sum", 1) => { let m = args[0].as_matrix(); let sum: f64 = m.data.iter().sum(); Ok(Value::Scalar(sum)) }
        ("max", 1) => { let m = args[0].as_matrix(); if m.data.is_empty() { return Ok(Value::Scalar(0.0)); } let mut max_val = f64::NEG_INFINITY; for &v in &m.data { if v > max_val { max_val = v; } } Ok(Value::Scalar(max_val)) }
        ("min", 1) => { let m = args[0].as_matrix(); if m.data.is_empty() { return Ok(Value::Scalar(0.0)); } let mut min_val = f64::INFINITY; for &v in &m.data { if v < min_val { min_val = v; } } Ok(Value::Scalar(min_val)) }

        ("std", 1) => {
            let m = args[0].as_matrix();
            let n = m.data.len() as f64;
            if n < 2.0 { return Ok(Value::Scalar(0.0)); }
            let mean = m.data.iter().sum::<f64>() / n;
            let variance = m.data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
            Ok(Value::Scalar(variance.sqrt()))
        }

        ("hamming", 1) => {
            let n = args[0].as_scalar().ok_or("hamming: argumen harus scalar")? as usize;
            if n <= 1 { return Ok(Value::Matrix(Matrix::new(1, 1, vec![1.0]))); }
            let mut data = Vec::with_capacity(n);
            for k in 0..n {
                let val = 0.54 - 0.46 * (2.0 * std::f64::consts::PI * k as f64 / (n as f64 - 1.0)).cos();
                data.push(val);
            }
            Ok(Value::Matrix(Matrix::new(1, n, data)))
        }

        ("xcorr", 2) => {
            let x = args[0].as_matrix();
            let y = args[1].as_matrix();
            if x.data.len() != y.data.len() { return Err("xcorr: panjang input harus sama".to_string()); }
            let n = x.data.len();
            let mut res = vec![0.0; 2 * n - 1];
            for lag in -(n as i32 - 1)..=(n as i32 - 1) {
                let mut sum = 0.0;
                for i in 0..n {
                    let j = i as i32 + lag;
                    if j >= 0 && j < n as i32 { sum += x.data[i] * y.data[j as usize]; }
                }
                res[(lag + (n as i32 - 1)) as usize] = sum;
            }
            Ok(Value::Matrix(Matrix::new(1, res.len(), res)))
        }

        ("filter", 3) => {
            let b = args[0].as_matrix(); let a = args[1].as_matrix(); let x = args[2].as_matrix();
            let nb = b.data.len(); let na = a.data.len(); let nx = x.data.len();
            
            if a.data.is_empty() || a.data[0] == 0.0 { return Err("filter: a[0] tidak boleh 0".to_string()); }
            if na > 1 { return Err("filter: untuk saat ini hanya mendukung FIR (panjang a == 1)".to_string()); }
            
            let mut y = vec![0.0; nx];
            for i in 0..nx {
                let mut sum = 0.0;
                for j in 0..nb { if i >= j { sum += b.data[j] * x.data[i - j]; } }
                y[i] = sum / a.data[0];
            }
            Ok(Value::Matrix(Matrix::new(1, nx, y)))
        }

        ("fft", 1) => {
            let m = args[0].as_matrix();
            let mut re: Vec<f64> = m.data.clone(); let mut im: Vec<f64> = vec![0.0; re.len()];
            let n = re.len();
            if n == 0 || (n & (n - 1)) != 0 { return Err("fft: panjang input harus kelipatan 2".to_string()); }
            fft_recursive(&mut re, &mut im);
            let mut data = Vec::with_capacity(2 * n); data.extend_from_slice(&re); data.extend_from_slice(&im);
            Ok(Value::Matrix(Matrix::new(2, n, data)))
        }

        ("ifft", 1) => {
            let m = args[0].as_matrix();
            if m.rows != 2 { return Err("ifft: input harus matriks 2xN".to_string()); }
            let n = m.cols;
            let mut re: Vec<f64> = m.data[0..n].to_vec(); let mut im: Vec<f64> = m.data[n..2*n].to_vec();
            for val in im.iter_mut() { *val = -*val; }
            fft_recursive(&mut re, &mut im);
            for i in 0..n { re[i] /= n as f64; im[i] = -(im[i] / n as f64); }
            let mut data = Vec::with_capacity(2 * n); data.extend_from_slice(&re); data.extend_from_slice(&im);
            Ok(Value::Matrix(Matrix::new(2, n, data)))
        }

        ("disp", 1) => match &args[0] { Value::Str(s) => return Ok(Value::Str(format!("{}\n", s))), Value::Scalar(n) => return Ok(Value::Str(format!("{}\n", n))), v => return Ok(Value::Str(format!("{}\n", v))), },
        ("cumsum", 1) => { let m = args[0].as_matrix(); let mut data = Vec::with_capacity(m.data.len()); let mut sum = 0.0; for &v in &m.data { sum += v; data.push(sum); } Ok(Value::Matrix(Matrix::new(1, data.len(), data))) }
        
        _ => Err(format!("Fungsi '{}' dengan {} argumen belum didukung", name, args.len())),
    }
}

fn unary_fn<F>(name: &str, arg: &Value, f: F) -> Result<Value, String>
where F: Fn(f64) -> f64 {
    match arg {
        Value::Scalar(x) => Ok(Value::Scalar(f(*x))),
        Value::Matrix(m) => { let mut result = m.clone(); for v in result.data.iter_mut() { *v = f(*v); } Ok(Value::Matrix(result)) }
        Value::Str(_) => Err(format!("{}: tidak bisa memproses tipe String", name)),
        _ => Err(format!("{}: tipe data tidak didukung", name)),
    }.map_err(|e: String| format!("{}: {}", name, e))
}

fn fft_recursive(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    if n <= 1 { return; }
    let mut re_even = vec![0.0; n / 2];
    let mut im_even = vec![0.0; n / 2];
    let mut re_odd = vec![0.0; n / 2];
    let mut im_odd = vec![0.0; n / 2];
    for i in 0..n / 2 {
        re_even[i] = re[2 * i]; im_even[i] = im[2 * i];
        re_odd[i] = re[2 * i + 1]; im_odd[i] = im[2 * i + 1];
    }
    fft_recursive(&mut re_even, &mut im_even);
    fft_recursive(&mut re_odd, &mut im_odd);
    for i in 0..n / 2 {
        let angle = -2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        let wr = angle.cos(); let wi = angle.sin();
        let tr = wr * re_odd[i] - wi * im_odd[i];
        let ti = wr * im_odd[i] + wi * re_odd[i];
        re[i] = re_even[i] + tr; im[i] = im_even[i] + ti;
        re[i + n / 2] = re_even[i] - tr; im[i + n / 2] = im_even[i] - ti;
    }
}

fn invert_matrix(m: &Matrix) -> Result<Matrix, String> {
    if m.rows != m.cols { return Err("inv: matriks harus persegi".to_string()); }
    let n = m.rows;
    let mut aug = Matrix::zeros(n, n * 2);
    for r in 0..n { for c in 0..n { aug.set(r, c, m.get(r, c)); aug.set(r, c + n, if r == c { 1.0 } else { 0.0 }); } }
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n { if aug.get(k, i).abs() > aug.get(max_row, i).abs() { max_row = k; } }
        if aug.get(max_row, i).abs() < 1e-9 { return Err("inv: matriks singular".to_string()); }
        if i != max_row { for j in 0..(n * 2) { let tmp = aug.get(i, j); aug.set(i, j, aug.get(max_row, j)); aug.set(max_row, j, tmp); } }
        let pivot = aug.get(i, i);
        for j in 0..(n * 2) { aug.set(i, j, aug.get(i, j) / pivot); }
        for k in 0..n { if k != i { let factor = aug.get(k, i); for j in 0..(n * 2) { aug.set(k, j, aug.get(k, j) - factor * aug.get(i, j)); } } }
    }
    let mut inv = Matrix::zeros(n, n);
    for r in 0..n { for c in 0..n { inv.set(r, c, aug.get(r, c + n)); } }
    Ok(inv)
}

fn randn_scalar() -> f64 {
    let u1: f64 = rand::random::<f64>().max(f64::EPSILON);
    let u2: f64 = rand::random::<f64>();
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

fn determinant(m: &Matrix) -> Result<f64, String> {
    let n = m.rows;
    if n == 1 { return Ok(m.get(0, 0)); }
    if n == 2 { return Ok(m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0)); }
    let mut temp = m.clone(); let mut det = 1.0;
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n { if temp.get(k, i).abs() > temp.get(max_row, i).abs() { max_row = k; } }
        if temp.get(max_row, i).abs() < 1e-12 { return Ok(0.0); }
        if i != max_row { for j in 0..n { let tmp = temp.get(i, j); temp.set(i, j, temp.get(max_row, j)); temp.set(max_row, j, tmp); } det = -det; }
        let pivot = temp.get(i, i); det *= pivot;
        for k in i + 1..n { let factor = temp.get(k, i) / pivot; for j in i..n { temp.set(k, j, temp.get(k, j) - factor * temp.get(i, j)); } }
    }
    Ok(det)
}

fn matrix_rank(m: &Matrix) -> f64 {
    let mut temp = m.clone(); let mut rank = 0; let rows = temp.rows; let cols = temp.cols; let mut row = 0;
    for col in 0..cols {
        if row >= rows { break; }
        let mut max_row = row;
        for k in row + 1..rows { if temp.get(k, col).abs() > temp.get(max_row, col).abs() { max_row = k; } }
        if temp.get(max_row, col).abs() > 1e-9 {
            if row != max_row { for j in 0..cols { let tmp = temp.get(row, j); temp.set(row, j, temp.get(max_row, j)); temp.set(max_row, j, tmp); } }
            let pivot = temp.get(row, col);
            for k in row + 1..rows { let factor = temp.get(k, col) / pivot; for j in col..cols { temp.set(k, j, temp.get(k, j) - factor * temp.get(row, j)); } }
            rank += 1; row += 1;
        }
    }
    rank as f64
}

fn power_iteration(m: &Matrix, max_iter: usize) -> Result<f64, String> {
    let n = m.rows; let mut v = vec![1.0; n]; let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let mut new_v = vec![0.0; n];
        for i in 0..n { for j in 0..n { new_v[i] += m.get(i, j) * v[j]; } }
        let mut max_val = 0.0;
        for &val in &new_v { if val.abs() > max_val { max_val = val.abs(); } }
        if max_val < 1e-12 { break; }
        for i in 0..n { new_v[i] /= max_val; }
        let mut dot_new_v = 0.0; let mut dot_v = 0.0;
        for i in 0..n { dot_new_v += new_v[i] * v[i]; dot_v += v[i] * v[i]; }
        eigenvalue = dot_new_v / dot_v; v = new_v;
    }
    Ok(eigenvalue)
}