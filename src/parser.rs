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

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if let Token::Identifier(name) = self.peek().clone() {
            if self.tokens.get(self.pos + 1) == Some(&Token::Assign) {
                self.advance();
                self.advance();
                let value = self.parse_expr()?;
                if self.peek() != &Token::Eof {
                    return Err(format!("Token tidak terduga: {:?}", self.peek()));
                }
                return Ok(Stmt::Assign { name, value });
            }
        }

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
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
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
        Ok(Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        // Handle unary minus: -5, -(x+1)
        if self.peek() == &Token::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Binary {
                op: BinOp::Sub,
                left: Box::new(Expr::Number(0.0)),
                right: Box::new(operand),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        // Handle transpose berulang: A''
        while self.peek() == &Token::Transpose {
            self.advance();
            expr = Expr::Transpose(Box::new(expr));
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Identifier(name) => {
                // Cek apakah ini function call: identifier '(' args ')'
                if self.peek() == &Token::LParen {
                    self.advance(); // buang '('
                    let mut args = Vec::new();
                    if self.peek() != &Token::RParen {
                        args.push(self.parse_expr()?);
                        while self.peek() == &Token::Comma {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    match self.advance() {
                        Token::RParen => Ok(Expr::Call { name, args }),
                        t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                    }
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Token::RParen => Ok(expr),
                    t => Err(format!("Diharapkan ')', dapat {:?}", t)),
                }
            }
            Token::LBracket => self.parse_matrix_lit(),
            t => Err(format!(
                "Diharapkan angka, variabel, '(', atau '[', dapat {:?}",
                t
            )),
        }
    }

    fn parse_matrix_lit(&mut self) -> Result<Expr, String> {
        let mut rows: Vec<Vec<Expr>> = Vec::new();
        let mut current_row: Vec<Expr> = Vec::new();

        // Handle matriks kosong []
        if self.peek() == &Token::RBracket {
            self.advance();
            return Ok(Expr::MatrixLit { rows: Vec::new() });
        }

        loop {
            let expr = self.parse_expr()?;
            current_row.push(expr);

            match self.peek() {
                Token::Comma => {
                    self.advance();
                }
                Token::Semicolon => {
                    self.advance();
                    rows.push(current_row);
                    current_row = Vec::new();
                }
                Token::RBracket => {
                    self.advance();
                    rows.push(current_row);
                    break;
                }
                t => {
                    return Err(format!(
                        "Diharapkan ',', ';', atau ']' dalam matriks, dapat {:?}",
                        t
                    ));
                }
            }
        }

        // Validasi: semua baris harus sama panjang
        let cols = rows[0].len();
        for (i, row) in rows.iter().enumerate() {
            if row.len() != cols {
                return Err(format!(
                    "Baris {} memiliki {} kolom, sedangkan baris 0 memiliki {} kolom",
                    i,
                    row.len(),
                    cols
                ));
            }
        }

        Ok(Expr::MatrixLit { rows })
    }
}
