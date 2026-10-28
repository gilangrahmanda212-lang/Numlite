use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(f64),
    Matrix(Matrix),
    Complex(Complex), // <-- BARU
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Matrix { rows, cols, data }
    }
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![0.0; rows * cols] }
    }
    pub fn ones(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![1.0; rows * cols] }
    }
    pub fn eye(n: usize) -> Self {
        let mut m = Matrix::zeros(n, n);
        for i in 0..n { m.data[i * n + i] = 1.0; }
        m
    }
    pub fn get(&self, r: usize, c: usize) -> f64 { self.data[r * self.cols + c] }
    pub fn set(&mut self, r: usize, c: usize, v: f64) { self.data[r * self.cols + c] = v; }
    pub fn is_scalar(&self) -> bool { self.rows == 1 && self.cols == 1 }
}

impl Value {
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            Value::Scalar(n) => Some(*n),
            Value::Matrix(m) if m.is_scalar() => Some(m.data[0]),
            _ => None,
        }
    }
    pub fn as_complex(&self) -> Option<Complex> {
        match self {
            Value::Scalar(n) => Some(Complex::new(*n, 0.0)),
            Value::Complex(c) => Some(c.clone()),
            _ => None,
        }
    }
    pub fn as_matrix(&self) -> Matrix {
        match self {
            Value::Scalar(n) => Matrix::new(1, 1, vec![*n]),
            Value::Matrix(m) => m.clone(),
            Value::Complex(c) => Matrix::new(1, 1, vec![c.re]), // Fallback
            Value::Str(_) => Matrix::new(0, 0, vec![]),
        }
    }
}

fn format_scalar(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        let s = format!("{:.4}", n);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn format_complex(c: &Complex) -> String {
    let re_str = format_scalar(c.re);
    let im_str = format_scalar(c.im.abs());
    
    if c.im == 0.0 {
        re_str
    } else if c.re == 0.0 {
        if c.im > 0.0 { format!("{}i", im_str) } else { format!("-{}i", im_str) }
    } else {
        if c.im > 0.0 { format!("{} + {}i", re_str, im_str) } else { format!("{} - {}i", re_str, im_str) }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Scalar(n) => write!(f, "{}", format_scalar(*n)),
            Value::Complex(c) => write!(f, "{}", format_complex(c)), // <-- Menampilkan format a + bi
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Matrix(m) => {
                if m.is_scalar() { return write!(f, "{}", format_scalar(m.data[0])); }
                if m.rows == 0 || m.cols == 0 { return write!(f, "[]"); }
                let mut col_widths = vec![0usize; m.cols];
                for c in 0..m.cols {
                    for r in 0..m.rows {
                        let len = format_scalar(m.get(r, c)).len();
                        if len > col_widths[c] { col_widths[c] = len; }
                    }
                }
                for r in 0..m.rows {
                    write!(f, "  ")?;
                    for c in 0..m.cols {
                        if c > 0 { write!(f, "  ")?; }
                        let s = format_scalar(m.get(r, c));
                        write!(f, "{:>width$}", s, width = col_widths[c])?;
                    }
                    if r < m.rows - 1 { writeln!(f)?; }
                }
                Ok(())
            }
        }
    }
}