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

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    // Entry point: parse satu statement
    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        // Cek apakah ini assignment: Identifier '=' expr
        if let Token::Identifier(name) = self.peek().clone() {
            // Look ahead: token berikutnya harus '='
            if self.tokens.get(self.pos + 1) == Some(&Token::Assign) {
                self.advance(); // buang identifier
                self.advance(); // buang '='
                let value = self.parse_expr()?;
                if self.peek() != &Token::Eof {
                    return Err(format!("Token tidak terduga: {:?}", self.peek()));
                }
                return Ok(Stmt::Assign { name, value });
            }
        }

        // Kalau bukan assignment, parse sebagai expression
        let expr = self.parse_expr()?;
        if self.peek() != &Token::Eof {
            return Err(format!("Token tidak terduga: {:?}", self.peek()));
        }
        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Identifier(name) => Ok(Expr::Var(name)),
            Token::LParen => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Token::RParen => Ok(expr),
                    t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            }
            t => Err(format!("Diharapkan angka, variabel, atau '(', dapat {:?}", t)),
        }
    }
}
