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

    fn peek2(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
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
                // Hati-hati: '.' bisa jadi awal DotStar (.*)
                if c == '.' && !s.is_empty() {
                    // Cek apakah ini desimal atau operator .*
                    if let Some(next) = self.peek2() {
                        if !next.is_ascii_digit() {
                            break; // bukan desimal, berhenti
                        }
                    }
                }
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s.parse::<f64>()
            .map_err(|_| format!("Angka tidak valid: {}", s))
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
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
                '0'..='9' => {
                    let n = self.read_number()?;
                    Token::Number(n)
                }
                '.' => {
                    // '.' bisa jadi desimal (.5) atau DotStar/DotSlash/DotCaret
                    if let Some(next) = self.peek2() {
                        match next {
                            '*' => {
                                self.advance();
                                self.advance();
                                Token::DotStar
                            }
                            '/' => {
                                self.advance();
                                self.advance();
                                Token::DotSlash
                            }
                            '^' => {
                                self.advance();
                                self.advance();
                                Token::DotCaret
                            }
                            '0'..='9' => {
                                let n = self.read_number()?;
                                Token::Number(n)
                            }
                            _ => {
                                return Err(format!(
                                    "Karakter '.' tidak valid di posisi {}",
                                    self.pos
                                ));
                            }
                        }
                    } else {
                        return Err("Karakter '.' di akhir input".to_string());
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    Token::Identifier(ident)
                }
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '=' => { self.advance(); Token::Assign }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '[' => { self.advance(); Token::LBracket }
                ']' => { self.advance(); Token::RBracket }
                ',' => { self.advance(); Token::Comma }
                ';' => { self.advance(); Token::Semicolon }
                '\'' => { self.advance(); Token::Transpose }
                '^' => { self.advance(); Token::Caret }
                _ => return Err(format!("Karakter tidak dikenal: '{}'", c)),
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}
