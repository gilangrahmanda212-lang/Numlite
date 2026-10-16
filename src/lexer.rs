use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s.parse::<f64>()
            .map_err(|_| format!("Angka tidak valid: {}", s))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            let c = match self.peek() {
                Some(c) => c,
                None => {
                    tokens.push(Token::Eof);
                    break;
                }
            };

            let token = match c {
                '0'..='9' | '.' => {
                    let n = self.read_number()?;
                    Token::Number(n)
                }
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                _ => return Err(format!("Karakter tidak dikenal: '{}'", c)),
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}
