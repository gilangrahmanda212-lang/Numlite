#[derive(Debug, Clone, PartialEq)] 
pub enum Expr {
    Number(f64),
    String(String),
    Var(String),
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    MatrixLit { rows: Vec<Vec<Expr>> },
    Transpose(Box<Expr>),
    Call { name: String, args: Vec<Expr> },
    Range { start: Box<Expr>, step: Option<Box<Expr>>, end: Box<Expr> },
    Index { target: Box<Expr>, index: Box<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, EMul, EDiv, EPow, Pow,
    Eq, Neq, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone, PartialEq)] 
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)] 
pub enum Stmt {
    Assign { name: String, value: Expr },
    Expr(Expr),
    For { var: String, iter: Expr, body: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
    If { cond: Expr, then_body: Vec<Stmt>, else_body: Option<Vec<Stmt>> },
    Switch { expr: Expr, cases: Vec<(Expr, Vec<Stmt>)>, otherwise: Option<Vec<Stmt>> },
    Break,
    Continue,
    FunctionDef(FunctionDef),
    Plot { title: String, x: Expr, y: Expr },
}