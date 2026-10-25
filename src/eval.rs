use std::collections::HashMap;
use crate::ast::{BinOp, Expr, Stmt};
use crate::value::{Matrix, Value};

pub type Env = HashMap<String, Value>;

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Scalar(*n)),
        Expr::String(s) => Ok(Value::Str(s.clone())),
        Expr::Var(name) => env.get(name).cloned().ok_or_else(|| format!("Variabel '{}' belum didefinisikan", name)),
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
            call_builtin(name, &values)
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
            let index_val = eval(index, env)?;
            let idx_mat = index_val.as_matrix();

            let mut result_data = Vec::new();
            for &idx in &idx_mat.data {
                let i = (idx as usize).saturating_sub(1);
                if i < mat.data.len() {
                    result_data.push(mat.data[i]);
                } else {
                    return Err(format!("Index {} di luar batas array", idx));
                }
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

pub fn exec_stmt(stmt: &Stmt, env: &mut Env, capture_output: &mut String) -> Result<(), String> {
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
                    exec_stmt(&stmt, env, capture_output)?;
                } else {
                    break; 
                }
            }
            Ok(())
        }
        Stmt::Plot { .. } => Ok(()),
        Stmt::Expr(Expr::Call { name, .. }) if name == "subplot" || name == "figure" || name == "grid" || name == "clc" || name == "clear" || name == "close" => {
            Ok(())
        }
        Stmt::Expr(expr) => {
            // Cek apakah ini fungsi disp()
            if let Expr::Call { name, args } = expr {
                if name == "disp" && args.len() == 1 {
                    let val = eval(&args[0], env)?;
                    match val {
                        Value::Str(s) => capture_output.push_str(&format!("{}\n", s)),
                        Value::Scalar(n) => capture_output.push_str(&format!("{}\n", n)),
                        v => capture_output.push_str(&format!("{}\n", v)),
                    }
                    return Ok(());
                }
            }

            let val = eval(expr, env)?;
            match &val {
                Value::Scalar(n) => capture_output.push_str(&format!("ans = {}\n", n)),
                _ => capture_output.push_str(&format!("ans =\n{}\n", val)),
            }
            Ok(())
        }
        Stmt::Assign { name, value } => {
            let v = eval(value, env)?;
            // Mencetak nama variabel dan nilainya
            match &v {
                Value::Scalar(n) => capture_output.push_str(&format!("{} =\n  {}\n", name, n)),
                _ => capture_output.push_str(&format!("{} =\n{}\n", name, v)),
            }
            env.insert(name.clone(), v);
            Ok(())
        }
        Stmt::For { var, iter, body } => {
            let iter_val = eval(iter, env)?;
            for val in iter_val.as_matrix().data {
                env.insert(var.clone(), Value::Scalar(val));
                for s in body { exec_stmt(s, env, capture_output)?; }
            }
            Ok(())
        }
        Stmt::If { cond, then_body, else_body } => {
            let cond_val = eval(cond, env)?;
            let is_true = cond_val.as_scalar().unwrap_or(0.0) != 0.0;
            if is_true {
                for s in then_body { exec_stmt(s, env, capture_output)?; }
            } else if let Some(eb) = else_body {
                for s in eb { exec_stmt(s, env, capture_output)?; }
            }
            Ok(())
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
        ("length", 1) => { let m = args[0].as_matrix(); Ok(Value::Scalar(m.data.len().max(m.rows).max(m.cols) as f64)) }
        ("size", 1) => { let m = args[0].as_matrix(); Ok(Value::Matrix(Matrix::new(1, 2, vec![m.rows as f64, m.cols as f64]))) }
        
        ("sqrt", 1) => unary_fn("sqrt", &args[0], |x| x.sqrt()),
        ("abs", 1) => unary_fn("abs", &args[0], |x| x.abs()),
        ("sin", 1) => unary_fn("sin", &args[0], |x| x.sin()),
        ("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
        ("tan", 1) => unary_fn("tan", &args[0], |x| x.tan()),
        ("exp", 1) => unary_fn("exp", &args[0], |x| x.exp()),
        ("log", 1) => unary_fn("log", &args[0], |x| x.ln()),

        ("linspace", 3) => {
            let start = args[0].as_scalar().ok_or("linspace: start harus scalar")?;
            let end = args[1].as_scalar().ok_or("linspace: end harus scalar")?;
            let n = args[2].as_scalar().ok_or("linspace: n harus scalar")? as usize;
            if n < 2 { return Ok(Value::Matrix(Matrix::new(1, 1, vec![start]))); }
            let mut data = Vec::with_capacity(n);
            let step = (end - start) / (n as f64 - 1.0);
            for i in 0..n { data.push(start + step * i as f64); }
            Ok(Value::Matrix(Matrix::new(1, n, data)))
        }
        
        ("rand", 1) => {
            let n = args[0].as_scalar().ok_or("rand: argumen harus scalar")? as usize;
            let mut data = Vec::with_capacity(n * n);
            for _ in 0..(n * n) { data.push(rand::random::<f64>()); }
            Ok(Value::Matrix(Matrix::new(n, n, data)))
        }
        ("rand", 2) => {
            let r = args[0].as_scalar().ok_or("rand: baris harus scalar")? as usize;
            let c = args[1].as_scalar().ok_or("rand: kolom harus scalar")? as usize;
            let mut data = Vec::with_capacity(r * c);
            for _ in 0..(r * c) { data.push(rand::random::<f64>()); }
            Ok(Value::Matrix(Matrix::new(r, c, data)))
        }

        ("conv", 2) => {
            let a = args[0].as_matrix();
            let b = args[1].as_matrix();
            if (a.rows > 1 && a.cols > 1) || (b.rows > 1 && b.cols > 1) {
                return Err("conv: input harus vektor 1D".to_string());
            }
            let vec_a = &a.data;
            let vec_b = &b.data;
            let out_len = vec_a.len() + vec_b.len() - 1;
            let mut result = vec![0.0; out_len];
            for i in 0..vec_a.len() {
                for j in 0..vec_b.len() { result[i + j] += vec_a[i] * vec_b[j]; }
            }
            Ok(Value::Matrix(Matrix::new(1, out_len, result)))
        }

        ("mean", 1) => {
            let m = args[0].as_matrix();
            if m.data.is_empty() { return Ok(Value::Scalar(0.0)); }
            let sum: f64 = m.data.iter().sum();
            Ok(Value::Scalar(sum / m.data.len() as f64))
        }
        ("sum", 1) => {
            let m = args[0].as_matrix();
            let sum: f64 = m.data.iter().sum();
            Ok(Value::Scalar(sum))
        }

        ("max", 1) => {
            let m = args[0].as_matrix();
            if m.data.is_empty() { return Ok(Value::Scalar(0.0)); }
            let mut max_val = f64::NEG_INFINITY;
            for &v in &m.data {
                if v > max_val { max_val = v; }
            }
            Ok(Value::Scalar(max_val))
        }

        ("fft", 1) => {
            let m = args[0].as_matrix();
            let mut re: Vec<f64> = m.data.clone();
            let mut im: Vec<f64> = vec![0.0; re.len()];
            let n = re.len();
            if n == 0 || (n & (n - 1)) != 0 {
                return Err("fft: panjang input harus kelipatan 2".to_string());
            }
            fft_recursive(&mut re, &mut im);
            let mut data = Vec::with_capacity(2 * n);
            data.extend_from_slice(&re);
            data.extend_from_slice(&im);
            Ok(Value::Matrix(Matrix::new(2, n, data)))
        }

        ("ifft", 1) => {
            let m = args[0].as_matrix();
            if m.rows != 2 { return Err("ifft: input harus matriks 2xN".to_string()); }
            let n = m.cols;
            let mut re: Vec<f64> = m.data[0..n].to_vec();
            let mut im: Vec<f64> = m.data[n..2*n].to_vec();
            for val in im.iter_mut() { *val = -*val; }
            fft_recursive(&mut re, &mut im);
            for i in 0..n {
                re[i] /= n as f64;
                im[i] = -(im[i] / n as f64);
            }
            let mut data = Vec::with_capacity(2 * n);
            data.extend_from_slice(&re);
            data.extend_from_slice(&im);
            Ok(Value::Matrix(Matrix::new(2, n, data)))
        }

        ("disp", 1) => {
            match &args[0] {
                Value::Str(s) => return Ok(Value::Str(format!("{}\n", s))),
                Value::Scalar(n) => return Ok(Value::Str(format!("{}\n", n))),
                v => return Ok(Value::Str(format!("{}\n", v))),
            }
        }
        ("cumsum", 1) => {
            let m = args[0].as_matrix();
            let mut data = Vec::with_capacity(m.data.len());
            let mut sum = 0.0;
            for &v in &m.data {
                sum += v;
                data.push(sum);
            }
            Ok(Value::Matrix(Matrix::new(1, data.len(), data)))
        }
        
        _ => Err(format!("Fungsi '{}' dengan {} argumen belum didukung", name, args.len())),
    }
}

fn unary_fn<F>(name: &str, arg: &Value, f: F) -> Result<Value, String>
where F: Fn(f64) -> f64,
{
    match arg {
        Value::Scalar(x) => Ok(Value::Scalar(f(*x))),
        Value::Matrix(m) => {
            let mut result = m.clone();
            for v in result.data.iter_mut() { *v = f(*v); }
            Ok(Value::Matrix(result))
        }
        Value::Str(_) => Err(format!("{}: tidak bisa memproses tipe String", name)),
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
        re_even[i] = re[2 * i];
        im_even[i] = im[2 * i];
        re_odd[i] = re[2 * i + 1];
        im_odd[i] = im[2 * i + 1];
    }
    fft_recursive(&mut re_even, &mut im_even);
    fft_recursive(&mut re_odd, &mut im_odd);
    for i in 0..n / 2 {
        let angle = -2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        let wr = angle.cos();
        let wi = angle.sin();
        let tr = wr * re_odd[i] - wi * im_odd[i];
        let ti = wr * im_odd[i] + wi * re_odd[i];
        re[i] = re_even[i] + tr;
        im[i] = im_even[i] + ti;
        re[i + n / 2] = re_even[i] - tr;
        im[i + n / 2] = im_even[i] - ti;
    }
}
