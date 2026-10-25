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
        rows: Vec<Vec<Expr>>,
    },
    Transpose(Box<Expr>),
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, EMul, EDiv, EPow, Pow,
    Eq, Neq, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign { name: String, value: Expr },
    Expr(Expr),
    For {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
}
