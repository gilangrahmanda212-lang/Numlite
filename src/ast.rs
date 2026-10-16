#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Var(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

// Statement: sesuatu yang tidak menghasilkan nilai (misal assignment)
#[derive(Debug, Clone)]
pub enum Stmt {
    Assign { name: String, value: Expr },
    Expr(Expr),
}
