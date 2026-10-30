use crate::ast::{BinOp, Expr, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current_title: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, current_title: String::new() }
    }

    pub fn is_at_eof(&self) -> bool { self.peek() == &Token::Eof }
    fn peek(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&Token::Eof) }
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() { self.pos += 1; }
        tok
    }
    fn skip_newlines(&mut self) {
        while self.peek() == &Token::Newline || self.peek() == &Token::Semicolon { self.advance(); }
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        if self.peek() == &Token::Eof { return Err("Eof".to_string()); }

        if self.peek() == &Token::Function {
            self.advance();
            let mut name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err("Diharapkan nama fungsi setelah 'function'".to_string()),
            };
            if self.peek() == &Token::Assign {
                self.advance();
                name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err("Diharapkan nama fungsi setelah '='".to_string()),
                };
            }
            let mut params = Vec::new();
            if self.peek() == &Token::LParen {
                self.advance();
                if self.peek() != &Token::RParen {
                    loop {
                        match self.advance() {
                            Token::Identifier(p) => params.push(p),
                            _ => return Err("Diharapkan nama parameter".to_string()),
                        }
                        if self.peek() == &Token::Comma { self.advance(); } else { break; }
                    }
                }
                if self.advance() != Token::RParen { return Err("Diharapkan ')' setelah parameter".to_string()); }
            }
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek() == &Token::End { self.advance(); break; }
                if self.peek() == &Token::Eof { return Err("Diharapkan 'end' untuk menutup fungsi".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt::FunctionDef(crate::ast::FunctionDef { name, params, body }));
        }

        if self.peek() == &Token::For {
            self.advance();
            let var = match self.advance() {
                Token::Identifier(name) => name,
                _ => return Err("Diharapkan nama variabel setelah 'for'".to_string()),
            };
            if self.advance() != Token::Assign {
                return Err("Diharapkan '=' setelah variabel loop".to_string());
            }
            let iter = self.parse_range()?;
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek() == &Token::End { self.advance(); break; }
                if self.peek() == &Token::Eof { return Err("Diharapkan 'end'".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt::For { var, iter, body });
        }

        if self.peek() == &Token::While {
            self.advance();
            let cond = self.parse_range()?;
            self.skip_newlines();
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.peek() == &Token::End { self.advance(); break; }
                if self.peek() == &Token::Eof { return Err("Diharapkan 'end'".to_string()); }
                body.push(self.parse_stmt()?);
            }
            return Ok(Stmt::While { cond, body });
        }

        if self.peek() == &Token::If {
            self.advance();
            let cond = self.parse_range()?;
            self.skip_newlines();
            let mut then_body = Vec::new();
            let mut else_body = None;
            loop {
                self.skip_newlines();
                if self.peek() == &Token::Else {
                    self.advance();
                    self.skip_newlines();
                    let mut eb = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek() == &Token::End { self.advance(); break; }
                        if self.peek() == &Token::Eof { return Err("Diharapkan 'end'".to_string()); }
                        eb.push(self.parse_stmt()?);
                    }
                    else_body = Some(eb);
                    break;
                }
                if self.peek() == &Token::End { self.advance(); break; }
                if self.peek() == &Token::Eof { return Err("Diharapkan 'end'".to_string()); }
                then_body.push(self.parse_stmt()?);
            }
            return Ok(Stmt::If { cond, then_body, else_body });
        }

        if self.peek() == &Token::Switch {
            self.advance();
            let expr = self.parse_range()?;
            self.skip_newlines();
            let mut cases = Vec::new();
            let mut otherwise = None;
            loop {
                self.skip_newlines();
                if self.peek() == &Token::Case {
                    self.advance();
                    let case_expr = self.parse_range()?;
                    self.skip_newlines();
                    let mut body = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek() == &Token::Case || self.peek() == &Token::Otherwise || self.peek() == &Token::End { break; }
                        body.push(self.parse_stmt()?);
                    }
                    cases.push((case_expr, body));
                } else if self.peek() == &Token::Otherwise {
                    self.advance();
                    self.skip_newlines();
                    let mut body = Vec::new();
                    loop {
                        self.skip_newlines();
                        if self.peek() == &Token::End { break; }
                        body.push(self.parse_stmt()?);
                    }
                    otherwise = Some(body);
                } else if self.peek() == &Token::End {
                    self.advance();
                    break;
                } else {
                    return Err("Diharapkan 'case', 'otherwise', atau 'end' di dalam switch".to_string());
                }
            }
            return Ok(Stmt::Switch { expr, cases, otherwise });
        }

        if self.peek() == &Token::Break { self.advance(); return Ok(Stmt::Break); }
        if self.peek() == &Token::Continue { self.advance(); return Ok(Stmt::Continue); }

        if let Token::Identifier(name) = self.peek().clone() {
            let next_tok = self.tokens.get(self.pos + 1);
            if next_tok == Some(&Token::LParen) {
                self.advance(); self.advance();
                let mut args = Vec::new();
                if self.peek() != &Token::RParen {
                    args.push(self.parse_range()?);
                    while self.peek() == &Token::Comma {
                        self.advance();
                        args.push(self.parse_range()?);
                    }
                }
                match self.advance() {
                    Token::RParen => {
                        match name.as_str() {
                            "title" => {
                                if let Some(Expr::String(s)) = args.get(0) { self.current_title = s.clone(); }
                                return Ok(Stmt::Expr(Expr::Number(0.0)));
                            }
                            "plot" => {
                                if args.len() >= 2 {
                                    let title = self.current_title.clone();
                                    self.current_title = String::new();
                                    return Ok(Stmt::Plot { title, x: args[0].clone(), y: args[1].clone() });
                                }
                            }
                            _ => {}
                        }
                        return Ok(Stmt::Expr(Expr::Call { name, args }));
                    }
                    t => return Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            } else if next_tok == Some(&Token::Assign) {
                self.advance(); self.advance();
                let value = self.parse_range()?;
                return Ok(Stmt::Assign { name, value });
            }
        }

        let expr = self.parse_range()?;
        Ok(Stmt::Expr(expr))
    }

    pub fn parse_range(&mut self) -> Result<Expr, String> {
        let mut start = self.parse_expr()?;
        while self.peek() == &Token::Colon {
            self.advance();
            let next = self.parse_expr()?;
            if self.peek() == &Token::Colon {
                self.advance();
                let end = self.parse_expr()?;
                start = Expr::Range { start: Box::new(start), step: Some(Box::new(next)), end: Box::new(end) };
            } else {
                start = Expr::Range { start: Box::new(start), step: None, end: Box::new(next) };
            }
        }
        Ok(start)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                Token::Eq => BinOp::Eq,
                Token::Neq => BinOp::Neq,
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::Le => BinOp::Le,
                Token::Ge => BinOp::Ge,
                Token::And => BinOp::And,
                Token::Or => BinOp::Or,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::DotStar => BinOp::EMul,
                Token::DotSlash => BinOp::EDiv,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;
        let op = match self.peek() {
            Token::Caret => BinOp::Pow,
            Token::DotCaret => BinOp::EPow,
            _ => return Ok(left),
        };
        self.advance();
        let right = self.parse_unary()?;
        Ok(Expr::Binary { op, left: Box::new(left), right: Box::new(right) })
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.peek() == &Token::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Binary { op: BinOp::Sub, left: Box::new(Expr::Number(0.0)), right: Box::new(operand) });
        }
        if self.peek() == &Token::Not {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Not(Box::new(operand)));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        while let Token::String(s) = self.peek() {
            if s.is_empty() {
                self.advance();
                expr = Expr::Transpose(Box::new(expr));
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::String(s) => {
                if s.is_empty() { Ok(Expr::Var("end".to_string())) } else { Ok(Expr::String(s)) }
            },
            Token::Identifier(name) => {
                if self.peek() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek() != &Token::RParen {
                        args.push(self.parse_range()?);
                        while self.peek() == &Token::Comma {
                            self.advance();
                            args.push(self.parse_range()?);
                        }
                    }
                    match self.advance() {
                        Token::RParen => {
                            if args.len() == 1 {
                                if let Expr::Var(ref v) = args[0] {
                                    if v == "end" {
                                        return Ok(Expr::Index { target: Box::new(Expr::Var(name)), index: Box::new(Expr::Call { name: "get_end".to_string(), args: vec![] }) });
                                    }
                                }
                                if let Expr::Range { .. } = args[0] {
                                    return Ok(Expr::Index { target: Box::new(Expr::Var(name)), index: Box::new(args[0].clone()) });
                                }
                            }
                            Ok(Expr::Call { name, args })
                        }
                        t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                    }
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_range()?;
                match self.advance() {
                    Token::RParen => Ok(expr),
                    t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            }
            Token::LBracket => self.parse_matrix_lit(),
            Token::Transpose => Ok(Expr::String("".to_string())),
            Token::End => Ok(Expr::Var("end".to_string())),
            t => Err(format!("Token tidak terduga: {:?}", t)),
        }
    }

    fn parse_matrix_lit(&mut self) -> Result<Expr, String> {
        let mut rows: Vec<Vec<Expr>> = Vec::new();
        let mut current_row: Vec<Expr> = Vec::new();
        if self.peek() == &Token::RBracket {
            self.advance();
            return Ok(Expr::MatrixLit { rows: Vec::new() });
        }
        loop {
            let expr = self.parse_range()?;
            current_row.push(expr);
            match self.peek() {
                Token::Comma => { self.advance(); }
                Token::Semicolon => { self.advance(); rows.push(current_row); current_row = Vec::new(); }
                Token::RBracket => { self.advance(); rows.push(current_row); break; }
                Token::Newline => { self.advance(); rows.push(current_row); current_row = Vec::new(); }
                t => return Err(format!("Diharapkan ',', ';', atau ']', dapat {:?}", t)),
            }
        }
        let is_concat = rows.iter().any(|row| row.iter().any(|e| matches!(e, Expr::Var(_) | Expr::Call { .. })));
        if is_concat {
            let mut row_exprs = Vec::new();
            for row in rows {
                if row.is_empty() { continue; }
                let mut current_expr = row[0].clone();
                for i in 1..row.len() {
                    current_expr = Expr::Call { name: "horzcat".to_string(), args: vec![current_expr, row[i].clone()] };
                }
                row_exprs.push(current_expr);
            }
            if row_exprs.is_empty() { return Ok(Expr::MatrixLit { rows: Vec::new() }); }
            let mut final_expr = row_exprs[0].clone();
            for i in 1..row_exprs.len() {
                final_expr = Expr::Call { name: "vertcat".to_string(), args: vec![final_expr, row_exprs[i].clone()] };
            }
            return Ok(final_expr);
        }
        let cols = rows[0].len();
        for row in &rows {
            if row.len() != cols { return Err("Jumlah kolom tidak sama".to_string()); }
        }
        Ok(Expr::MatrixLit { rows })
    }
}