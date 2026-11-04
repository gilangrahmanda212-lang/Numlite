use crate::ast::{BinOp, Expr, ExprKind, PlotType, Stmt, StmtKind};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current_title: String,
    current_xlabel: String,
    current_ylabel: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, current_title: String::new(), current_xlabel: String::new(), current_ylabel: String::new() }
    }

    pub fn current_line(&self) -> usize { 
        self.peek().line 
    }

    pub fn is_at_eof(&self) -> bool { 
        self.peek().kind == TokenKind::Eof 
    }
    
    fn peek(&self) -> &Token { 
        self.tokens.get(self.pos).unwrap_or(&Token { kind: TokenKind::Eof, line: 0 }) 
    }
    
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() { self.pos += 1; }
        tok
    }
    
    fn skip_newlines(&mut self) {
        while self.peek().kind == TokenKind::Newline || self.peek().kind == TokenKind::Semicolon { 
            self.advance(); 
        }
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        if self.peek().kind == TokenKind::Eof { return Err("Eof".to_string()); }
        
        let line = self.peek().line;

        // --- DETEKSI STRUCT ASSIGNMENT ---
        if let TokenKind::Identifier(name) = self.peek().kind.clone() {
            if self.tokens.get(self.pos + 1).map(|t| &t.kind) == Some(&TokenKind::Dot) {
                self.advance(); 
                let mut fields = Vec::new();
                while self.peek().kind == TokenKind::Dot {
                    self.advance(); 
                    match self.advance().kind {
                        TokenKind::Identifier(f) => fields.push(f),
                        _ => return Err("Diharapkan nama field setelah titik".to_string()),
                    };
                }
                if self.advance().kind != TokenKind::Assign {
                    return Err("Diharapkan '=' setelah nama field".to_string());
                }
                let value = self.parse_range()?;
                return Ok(Stmt { kind: StmtKind::StructAssign { object: name, field: fields.join("."), value }, line });
            }
        }

        // --- DETEKSI HOLD ---
        if let TokenKind::Identifier(id) = self.peek().kind.clone() {
            if id == "hold" {
                self.advance();
                match self.advance().kind {
                    TokenKind::Identifier(action) => {
                        if action == "on" { return Ok(Stmt { kind: StmtKind::Hold(true), line }); }
                        else if action == "off" { return Ok(Stmt { kind: StmtKind::Hold(false), line }); }
                        else { return Err("Diharapkan 'on' atau 'off' setelah 'hold'".to_string()); }
                    }
                    _ => return Err("Diharapkan 'on' atau 'off' setelah 'hold'".to_string()),
                }
            }
        }

        // --- DETEKSI FUNCTION ---
        if self.peek().kind == TokenKind::Function {
            self.advance();
            let mut name = match self.advance().kind {
                TokenKind::Identifier(n) => n,
                _ => return Err("Diharapkan nama fungsi setelah 'function'".to_string()),
            };
            if self.peek().kind == TokenKind::Assign {
                self.advance();
                name = match self.advance().kind {
                    TokenKind::Identifier(n) => n,
                    _ => return Err("Diharapkan nama fungsi setelah '='".to_string()),
                };
            }
            let mut params = Vec::new();
            if self.peek().kind == TokenKind::LParen {
                self.advance();
                if self.peek().kind != TokenKind::RParen {
                    loop {
                        match self.advance().kind {
                            TokenKind::Identifier(p) => params.push(p),
                            _ => return Err("Diharapkan nama parameter".to_string()),
                        }
                        if self.peek().kind == TokenKind::Comma { self.advance(); } else { break; }
                    }
                }
                if self.advance().kind != TokenKind::RParen { return Err("Diharapkan ')' setelah parameter".to_string()); }
            }
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek().kind == TokenKind::End { self.advance(); break; }
                if self.peek().kind == TokenKind::Eof { return Err("Diharapkan 'end' untuk menutup fungsi".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt { kind: StmtKind::FunctionDef(crate::ast::FunctionDef { name, params, body }), line });
        }

        // --- DETEKSI FOR ---
        if self.peek().kind == TokenKind::For {
            self.advance();
            let var = match self.advance().kind {
                TokenKind::Identifier(name) => name,
                _ => return Err("Diharapkan nama variabel setelah 'for'".to_string()),
            };
            if self.advance().kind != TokenKind::Assign {
                return Err("Diharapkan '=' setelah variabel loop".to_string());
            }
            let iter = self.parse_range()?;
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek().kind == TokenKind::End { self.advance(); break; }
                if self.peek().kind == TokenKind::Eof { return Err("Diharapkan 'end'".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt { kind: StmtKind::For { var, iter, body }, line });
        }

        // --- DETEKSI WHILE ---
        if self.peek().kind == TokenKind::While {
            self.advance();
            let cond = self.parse_range()?;
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek().kind == TokenKind::End { self.advance(); break; }
                if self.peek().kind == TokenKind::Eof { return Err("Diharapkan 'end'".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt { kind: StmtKind::While { cond, body }, line });
        }

        // --- DETEKSI IF ---
        if self.peek().kind == TokenKind::If {
            self.advance();
            let cond = self.parse_range()?;
            self.skip_newlines();
            let mut then_body = Vec::new();
            let mut else_body = None;
            loop {
                self.skip_newlines();
                if self.peek().kind == TokenKind::Else {
                    self.advance();
                    self.skip_newlines();
                    let mut eb = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek().kind == TokenKind::End { self.advance(); break; }
                        if self.peek().kind == TokenKind::Eof { return Err("Diharapkan 'end'".to_string()); }
                        eb.push(self.parse_stmt()?);
                    }
                    else_body = Some(eb);
                    break;
                }
                if self.peek().kind == TokenKind::End { self.advance(); break; }
                if self.peek().kind == TokenKind::Eof { return Err("Diharapkan 'end'".to_string()); }
                then_body.push(self.parse_stmt()?);
            }
            return Ok(Stmt { kind: StmtKind::If { cond, then_body, else_body }, line });
        }

        // --- DETEKSI SWITCH ---
        if self.peek().kind == TokenKind::Switch {
            self.advance();
            let expr = self.parse_range()?;
            self.skip_newlines();
            let mut cases = Vec::new();
            let mut otherwise = None;
            loop {
                self.skip_newlines();
                if self.peek().kind == TokenKind::Case {
                    self.advance();
                    let case_expr = self.parse_range()?;
                    self.skip_newlines();
                    let mut body = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek().kind == TokenKind::Case || self.peek().kind == TokenKind::Otherwise || self.peek().kind == TokenKind::End { break; }
                        body.push(self.parse_stmt()?);
                    }
                    cases.push((case_expr, body));
                } else if self.peek().kind == TokenKind::Otherwise {
                    self.advance();
                    self.skip_newlines();
                    let mut body = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek().kind == TokenKind::End { break; }
                        body.push(self.parse_stmt()?);
                    }
                    otherwise = Some(body);
                } else if self.peek().kind == TokenKind::End {
                    self.advance();
                    break;
                } else {
                    return Err("Diharapkan 'case', 'otherwise', atau 'end' di dalam switch".to_string());
                }
            }
            return Ok(Stmt { kind: StmtKind::Switch { expr, cases, otherwise }, line });
        }

        if self.peek().kind == TokenKind::Break { self.advance(); return Ok(Stmt { kind: StmtKind::Break, line }); }
        if self.peek().kind == TokenKind::Continue { self.advance(); return Ok(Stmt { kind: StmtKind::Continue, line }); }

        // --- DETEKSI FUNGSI / ASSIGNMENT ---
        if let TokenKind::Identifier(name) = self.peek().kind.clone() {
            let next_tok = self.tokens.get(self.pos + 1);
            if next_tok.map(|t| &t.kind) == Some(&TokenKind::LParen) {
                self.advance(); self.advance();
                let mut args = Vec::new();
                if self.peek().kind != TokenKind::RParen {
                    args.push(self.parse_range()?);
                    while self.peek().kind == TokenKind::Comma {
                        self.advance();
                        args.push(self.parse_range()?);
                    }
                }
                match self.advance().kind {
                    TokenKind::RParen => {
                        match name.as_str() {
                            "title" => {
                                if let Some(Expr { kind: ExprKind::String(s), .. }) = args.get(0) { self.current_title = s.clone(); }
                                return Ok(Stmt { kind: StmtKind::Expr(Expr { kind: ExprKind::Call { name: "figure".to_string(), args: vec![] }, line }), line });
                            }
                            "xlabel" => {
                                if let Some(Expr { kind: ExprKind::String(s), .. }) = args.get(0) { self.current_xlabel = s.clone(); }
                                return Ok(Stmt { kind: StmtKind::Expr(Expr { kind: ExprKind::Call { name: "figure".to_string(), args: vec![] }, line }), line });
                            }
                            "ylabel" => {
                                if let Some(Expr { kind: ExprKind::String(s), .. }) = args.get(0) { self.current_ylabel = s.clone(); }
                                return Ok(Stmt { kind: StmtKind::Expr(Expr { kind: ExprKind::Call { name: "figure".to_string(), args: vec![] }, line }), line });
                            }
                            "plot" | "stem" => {
                                let (x_opt, y_expr) = if args.len() == 1 {
                                    (None, args[0].clone())
                                } else if args.len() == 2 {
                                    (Some(args[0].clone()), args[1].clone())
                                } else {
                                    return Err(format!("{}: butuh 1 atau 2 argumen", name));
                                };
                                
                                let title = self.current_title.clone();
                                let xlabel = self.current_xlabel.clone();
                                let ylabel = self.current_ylabel.clone();
                                self.current_title.clear(); self.current_xlabel.clear(); self.current_ylabel.clear();
                                
                                let plot_type = if name == "plot" { PlotType::Line } else { PlotType::Stem };
                                return Ok(Stmt { kind: StmtKind::Plot { title, xlabel, ylabel, plot_type, x: x_opt, y: y_expr }, line });
                            }
                            _ => {}
                        }
                        return Ok(Stmt { kind: StmtKind::Expr(Expr { kind: ExprKind::Call { name, args }, line }), line });
                    }
                    t => return Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            } else if next_tok.map(|t| &t.kind) == Some(&TokenKind::Assign) {
                self.advance(); self.advance();
                let value = self.parse_range()?;
                return Ok(Stmt { kind: StmtKind::Assign { name, value }, line });
            }
        }

        let expr = self.parse_range()?;
        Ok(Stmt { kind: StmtKind::Expr(expr), line })
    }

    pub fn parse_range(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut start = self.parse_expr()?;
        while self.peek().kind == TokenKind::Colon {
            self.advance();
            let next = self.parse_expr()?;
            if self.peek().kind == TokenKind::Colon {
                self.advance();
                let end = self.parse_expr()?;
                start = Expr { kind: ExprKind::Range { start: Box::new(start), step: Some(Box::new(next)), end: Box::new(end) }, line };
            } else {
                start = Expr { kind: ExprKind::Range { start: Box::new(start), step: None, end: Box::new(next) }, line };
            }
        }
        Ok(start)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Neq => BinOp::Neq,
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Ge => BinOp::Ge,
                TokenKind::And => BinOp::And,
                TokenKind::Or => BinOp::Or,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr { kind: ExprKind::Binary { op, left: Box::new(left), right: Box::new(right) }, line };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::DotStar => BinOp::EMul,
                TokenKind::DotSlash => BinOp::EDiv,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr { kind: ExprKind::Binary { op, left: Box::new(left), right: Box::new(right) }, line };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let left = self.parse_unary()?;
        let op = match self.peek().kind {
            TokenKind::Caret => BinOp::Pow,
            TokenKind::DotCaret => BinOp::EPow,
            _ => return Ok(left),
        };
        self.advance();
        let right = self.parse_unary()?;
        Ok(Expr { kind: ExprKind::Binary { op, left: Box::new(left), right: Box::new(right) }, line })
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        if self.peek().kind == TokenKind::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr { kind: ExprKind::Binary { op: BinOp::Sub, left: Box::new(Expr { kind: ExprKind::Number(0.0), line }), right: Box::new(operand) }, line });
        }
        if self.peek().kind == TokenKind::Not {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr { kind: ExprKind::Not(Box::new(operand)), line });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut expr = self.parse_factor()?;
        loop {
            if self.peek().kind == TokenKind::Transpose {
                self.advance();
                expr = Expr { kind: ExprKind::Transpose(Box::new(expr)), line };
            } else if let TokenKind::String(s) = &self.peek().kind {
                if s.is_empty() {
                    self.advance();
                    expr = Expr { kind: ExprKind::Transpose(Box::new(expr)), line };
                } else {
                    break;
                }
            } else if self.peek().kind == TokenKind::Dot {
                self.advance();
                let field = match self.advance().kind {
                    TokenKind::Identifier(f) => f,
                    _ => return Err("Diharapkan nama field setelah titik".to_string()),
                };
                expr = Expr { kind: ExprKind::StructAccess { object: Box::new(expr), field }, line };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        match self.advance().kind {
            TokenKind::Number(n) => Ok(Expr { kind: ExprKind::Number(n), line }),
            TokenKind::String(s) => {
                if s.is_empty() { Ok(Expr { kind: ExprKind::Var("end".to_string()), line }) } 
                else { Ok(Expr { kind: ExprKind::String(s), line }) }
            },
            TokenKind::Identifier(name) => {
                if self.peek().kind == TokenKind::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        args.push(self.parse_range()?);
                        while self.peek().kind == TokenKind::Comma {
                            self.advance();
                            args.push(self.parse_range()?);
                        }
                    }
                    match self.advance().kind {
                        TokenKind::RParen => {
                            if args.len() == 1 {
                                if let ExprKind::Var(v) = &args[0].kind {
                                    if v == "end" {
                                        return Ok(Expr { kind: ExprKind::Index { target: Box::new(Expr { kind: ExprKind::Var(name), line }), index: Box::new(Expr { kind: ExprKind::Call { name: "get_end".to_string(), args: vec![] }, line }) }, line });
                                    }
                                }
                                if let ExprKind::Range { .. } = args[0].kind {
                                    return Ok(Expr { kind: ExprKind::Index { target: Box::new(Expr { kind: ExprKind::Var(name), line }), index: Box::new(args[0].clone()) }, line });
                                }
                            }
                            Ok(Expr { kind: ExprKind::Call { name, args }, line })
                        }
                        t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                    }
                } else {
                    Ok(Expr { kind: ExprKind::Var(name), line })
                }
            }
            TokenKind::LParen => {
                let expr = self.parse_range()?;
                match self.advance().kind {
                    TokenKind::RParen => Ok(expr),
                    t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            }
            TokenKind::LBracket => self.parse_matrix_lit(),
            TokenKind::LBrace => self.parse_cell_lit(),
            TokenKind::End => Ok(Expr { kind: ExprKind::Var("end".to_string()), line }),
            t => Err(format!("Token tidak terduga: {:?}", t)),
        }
    }

    fn parse_cell_lit(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut elements = Vec::new();
        if self.peek().kind == TokenKind::RBrace {
            self.advance();
            return Ok(Expr { kind: ExprKind::CellLit { elements: Vec::new() }, line });
        }
        loop {
            let expr = self.parse_range()?;
            elements.push(expr);
            
            match &self.peek().kind {
                TokenKind::Comma => { self.advance(); }
                TokenKind::RBrace => { self.advance(); break; }
                t => return Err(format!("Diharapkan ',' atau '}}', dapat {:?}", t)),
            }
        }
        Ok(Expr { kind: ExprKind::CellLit { elements }, line })
    }

    fn parse_matrix_lit(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut rows: Vec<Vec<Expr>> = Vec::new();
        let mut current_row: Vec<Expr> = Vec::new();
        if self.peek().kind == TokenKind::RBracket {
            self.advance();
            return Ok(Expr { kind: ExprKind::MatrixLit { rows: Vec::new() }, line });
        }
        loop {
            let expr = self.parse_range()?;
            current_row.push(expr);
            
            match &self.peek().kind {
                TokenKind::Comma => { self.advance(); }
                TokenKind::Semicolon => { self.advance(); rows.push(current_row); current_row = Vec::new(); }
                TokenKind::RBracket => { self.advance(); rows.push(current_row); break; }
                TokenKind::Newline => { self.advance(); rows.push(current_row); current_row = Vec::new(); }
                t => return Err(format!("Diharapkan ',', ';', atau ']', dapat {:?}", t)),
            }
        }
        let is_concat = rows.iter().any(|row| row.iter().any(|e| matches!(e.kind, ExprKind::Var(_) | ExprKind::Call { .. })));
        if is_concat {
            let mut row_exprs = Vec::new();
            for row in rows {
                if row.is_empty() { continue; }
                let mut current_expr = row[0].clone();
                for i in 1..row.len() {
                    current_expr = Expr { kind: ExprKind::Call { name: "horzcat".to_string(), args: vec![current_expr, row[i].clone()] }, line };
                }
                row_exprs.push(current_expr);
            }
            if row_exprs.is_empty() { return Ok(Expr { kind: ExprKind::MatrixLit { rows: Vec::new() }, line }); }
            let mut final_expr = row_exprs[0].clone();
            for i in 1..row_exprs.len() {
                final_expr = Expr { kind: ExprKind::Call { name: "vertcat".to_string(), args: vec![final_expr, row_exprs[i].clone()] }, line };
            }
            return Ok(final_expr);
        }
        let cols = rows[0].len();
        for row in &rows {
            if row.len() != cols { return Err("Jumlah kolom tidak sama".to_string()); }
        }
        Ok(Expr { kind: ExprKind::MatrixLit { rows }, line })
    }
}