use crate::value::{Matrix, Value};
use crate::eval::format_string;
use crate::math_utils::*;

pub fn call_builtin(name: &str, args: &[Value]) -> Result<Value, String> {
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
        ("atan2", 2) => elementwise(&args[0], &args[1], |y, x| Ok(y.atan2(x))),
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
        
        ("strcat", _) => {
            let mut result = String::new();
            for arg in args {
                match arg {
                    Value::Str(s) => result.push_str(s),
                    v => result.push_str(&format!("{}", v)),
                }
            }
            Ok(Value::Str(result))
        }
        ("num2str", 1) => {
            match &args[0] {
                Value::Str(s) => Ok(Value::Str(s.clone())),
                v => Ok(Value::Str(format!("{}", v))),
            }
        }
        ("str2num", 1) => {
            if let Value::Str(s) = &args[0] {
                let cleaned = s.trim();
                match cleaned.parse::<f64>() {
                    Ok(n) => Ok(Value::Scalar(n)),
                    Err(_) => Err(format!("str2num: tidak bisa mengonversi '{}' ke angka", cleaned)),
                }
            } else { Err("str2num: argumen harus string".to_string()) }
        }
        ("sprintf", _) => {
            if args.is_empty() { return Err("sprintf: butuh argumen".to_string()); }
            if let Value::Str(fmt) = &args[0] {
                Ok(Value::Str(format_string(fmt, &args[1..])))
            } else {
                Err("sprintf: argumen pertama harus string".to_string())
            }
        }
        
        _ => Err(format!("Fungsi '{}' dengan {} argumen belum didukung", name, args.len())),
    }
}

pub fn unary_fn<F>(name: &str, arg: &Value, f: F) -> Result<Value, String>
where F: Fn(f64) -> f64 {
    match arg {
        Value::Scalar(x) => Ok(Value::Scalar(f(*x))),
        Value::Matrix(m) => {
            let mut result = m.clone();
            for v in result.data.iter_mut() { *v = f(*v); }
            Ok(Value::Matrix(result))
        }
        Value::Str(_) => Err(format!("{}: tidak bisa memproses tipe String", name)),
        _ => Err(format!("{}: tipe data tidak didukung", name)),
    }.map_err(|e: String| format!("{}: {}", name, e))
}

pub fn elementwise<F>(l: &Value, r: &Value, f: F) -> Result<Value, String>
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

pub fn matrix_mul(l: &Value, r: &Value) -> Result<Value, String> {
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
