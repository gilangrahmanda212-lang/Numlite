use crate::ast::{BinOp, Expr, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn is_at_eof(&self) -> bool {
        self.peek() == &Token::Eof
    }

    fn peek(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&Token::Eof) }
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() { self.pos += 1; }
        tok
    }
    fn skip_newlines(&mut self) {
        while self.peek() == &Token::Newline { self.advance(); }
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        if self.peek() == &Token::Eof { return Err("Eof".to_string()); }

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

        if let Token::Identifier(name) = self.peek().clone() {
            if self.tokens.get(self.pos + 1) == Some(&Token::Assign) {
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
                start = Expr::Range {
                    start: Box::new(start),
                    step: Some(Box::new(next)),
                    end: Box::new(end),
                };
            } else {
                start = Expr::Range {
                    start: Box::new(start),
                    step: None,
                    end: Box::new(next),
                };
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
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        while self.peek() == &Token::Transpose {
            self.advance();
            expr = Expr::Transpose(Box::new(expr));
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::String(s) => Ok(Expr::String(s)), // <-- Dukungan String untuk Script
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
                        Token::RParen => Ok(Expr::Call { name, args }),
                        t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                    }
                } else { Ok(Expr::Var(name)) }
            }
            Token::LParen => {
                let expr = self.parse_range()?;
                match self.advance() {
                    Token::RParen => Ok(expr),
                    t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            }
            Token::LBracket => self.parse_matrix_lit(),
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
                t => return Err(format!("Diharapkan ',', ';', atau ']', dapat {:?}", t)),
            }
        }
        let cols = rows[0].len();
        for row in &rows {
            if row.len() != cols { return Err("Jumlah kolom tidak sama".to_string()); }
        }
        Ok(Expr::MatrixLit { rows })
    }
}
