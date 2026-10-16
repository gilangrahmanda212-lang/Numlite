use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(f64),
    Matrix(Matrix),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,   // row-major order
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Panjang data tidak sesuai dimensi");
        Matrix { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }

    pub fn eye(n: usize) -> Self {
        let mut m = Matrix::zeros(n, n);
        for i in 0..n {
            m.data[i * n + i] = 1.0;
        }
        m
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]
    }

    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        self.data[r * self.cols + c] = v;
    }

    pub fn is_scalar(&self) -> bool {
        self.rows == 1 && self.cols == 1
    }
}

impl Value {
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            Value::Scalar(n) => Some(*n),
            Value::Matrix(m) if m.is_scalar() => Some(m.data[0]),
            _ => None,
        }
    }

    pub fn as_matrix(&self) -> Matrix {
        match self {
            Value::Scalar(n) => Matrix::new(1, 1, vec![*n]),
            Value::Matrix(m) => m.clone(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Scalar(n) => {
                // Kalau bilangan bulat, tampilkan tanpa .0
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Matrix(m) => {
                if m.is_scalar() {
                    return write!(f, "{}", m.data[0]);
                }
                for r in 0..m.rows {
                    for c in 0..m.cols {
                        if c > 0 {
                            write!(f, "  ")?;
                        }
                        let v = m.get(r, c);
                        if v.fract() == 0.0 && v.abs() < 1e15 {
                            write!(f, "{:>4}", v as i64)?;
                        } else {
                            write!(f, "{:>8.4}", v)?;
                        }
                    }
                    if r < m.rows - 1 {
                        writeln!(f)?;
                    }
                }
                Ok(())
            }
        }
    }
}
