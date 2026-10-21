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
        return Ok(Value::Matrix(Matrix::zeros(0, 0)));
    }
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut data = Vec::with_capacity(nrows * ncols);
    for row in rows {
        for cell in row {
            let v = eval(cell, env)?;
            let s = v.as_scalar().ok_or_else(|| {
                "Elemen matriks harus scalar".to_string()
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
            if b == 0.0 { Err("Pembagian dengan nol".to_string()) } else { Ok(a / b) }
        }),
        BinOp::EPow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        BinOp::Mul => matrix_mul(l, r),
        BinOp::Div => elementwise(l, r, |a, b| {
            if b == 0.0 { Err("Pembagian dengan nol".to_string()) } else { Ok(a / b) }
        }),
        BinOp::Pow => elementwise(l, r, |a, b| Ok(a.powf(b))),
    }
}

fn elementwise<F>(l: &Value, r: &Value, f: F) -> Result<Value, String>
where F: Fn(f64, f64) -> Result<f64, String>,
{
    if let (Value::Scalar(a), Value::Scalar(b)) = (l, r) {
        return f(*a, *b).map(Value::Scalar);
    }
    if let (Value::Scalar(a), Value::Matrix(m)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() { *v = f(*a, *v)?; }
        return Ok(Value::Matrix(result));
    }
    if let (Value::Matrix(m), Value::Scalar(b)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() { *v = f(*v, *b)?; }
        return Ok(Value::Matrix(result));
    }
    let (lm, rm) = (l.as_matrix(), r.as_matrix());
    if lm.rows != rm.rows || lm.cols != rm.cols {
        return Err(format!("Dimensi tidak cocok: {}x{} vs {}x{}", lm.rows, lm.cols, rm.rows, rm.cols));
    }
    let mut result = Matrix::zeros(lm.rows, lm.cols);
    for i in 0..lm.data.len() {
        result.data[i] = f(lm.data[i], rm.data[i])?;
    }
    Ok(Value::Matrix(result))
}

fn matrix_mul(l: &Value, r: &Value) -> Result<Value, String> {
    if let (Value::Matrix(a), Value::Matrix(b)) = (l, r) {
        if a.cols != b.rows {
            return Err(format!("Matrix multiplication tidak valid: {}x{} * {}x{}", a.rows, a.cols, b.rows, b.cols));
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
    if let (Value::Scalar(a), Value::Scalar(b)) = (l, r) {
        return Ok(Value::Scalar(a * b));
    }
    if let (Value::Scalar(s), Value::Matrix(m)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() { *v *= s; }
        return Ok(Value::Matrix(result));
    }
    if let (Value::Matrix(m), Value::Scalar(s)) = (l, r) {
        let mut result = m.clone();
        for v in result.data.iter_mut() { *v *= s; }
        return Ok(Value::Matrix(result));
    }
    Err("Kombinasi tipe tidak valid untuk *".to_string())
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
        
        // Fungsi Matematika Dasar
        ("sqrt", 1) => unary_fn("sqrt", &args[0], |x| x.sqrt()),
        ("abs", 1) => unary_fn("abs", &args[0], |x| x.abs()),
        ("sin", 1) => unary_fn("sin", &args[0], |x| x.sin()),
        ("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
        ("tan", 1) => unary_fn("tan", &args[0], |x| x.tan()),
        ("exp", 1) => unary_fn("exp", &args[0], |x| x.exp()),
        ("log", 1) => unary_fn("log", &args[0], |x| x.ln()),

        // --- FITUR MILESTONE 2 (Signal & Statistik Dasar) ---
        
        ("linspace", 3) => {
            let start = args[0].as_scalar().ok_or("linspace: start harus scalar")?;
            let end = args[1].as_scalar().ok_or("linspace: end harus scalar")?;
            let n = args[2].as_scalar().ok_or("linspace: n harus scalar")? as usize;
            
            if n < 2 {
                return Ok(Value::Matrix(Matrix::new(1, 1, vec![start])));
            }
            let mut data = Vec::with_capacity(n);
            let step = (end - start) / (n as f64 - 1.0);
            for i in 0..n {
                data.push(start + step * i as f64);
            }
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
                for j in 0..vec_b.len() {
                    result[i + j] += vec_a[i] * vec_b[j];
                }
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

        // 5. FFT (Fast Fourier Transform)
        ("fft", 1) => {
            let m = args[0].as_matrix();
            let mut re: Vec<f64> = m.data.clone();
            let mut im: Vec<f64> = vec![0.0; re.len()];
            let n = re.len();

            if n == 0 || (n & (n - 1)) != 0 {
                return Err("fft: panjang input harus kelipatan 2 (2, 4, 8, 16, dst)".to_string());
            }

            fft_recursive(&mut re, &mut im);

            let mut data = Vec::with_capacity(2 * n);
            data.extend_from_slice(&re);
            data.extend_from_slice(&im);

            Ok(Value::Matrix(Matrix::new(2, n, data)))
        }

        // 6. IFFT (Inverse Fast Fourier Transform)
        ("ifft", 1) => {
            let m = args[0].as_matrix();
            if m.rows != 2 {
                return Err("ifft: input harus matriks 2xN (baris 1 real, baris 2 imaginer)".to_string());
            }
            let n = m.cols;
            let mut re: Vec<f64> = m.data[0..n].to_vec();
            let mut im: Vec<f64> = m.data[n..2*n].to_vec();

            for val in im.iter_mut() {
                *val = -*val;
            }

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
