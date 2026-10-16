#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Var(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    MatrixLit {
        rows: Vec<Vec<Expr>>,   // baris demi baris
    },
    Transpose(Box<Expr>),
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,       // matrix multiplication
    Div,       // matrix division (right-div, nanti)
    EMul,      // .*
    EDiv,      // ./
    EPow,      // .^
    Pow,       // ^
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign { name: String, value: Expr },
    Expr(Expr),
}
