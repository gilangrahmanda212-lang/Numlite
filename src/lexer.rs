use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { input: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> { self.input.get(self.pos).copied() }
    fn peek2(&self) -> Option<char> { self.input.get(self.pos + 1).copied() }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() { self.pos += 1; }
        c
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                if c == '.' && !s.is_empty() {
                    if let Some(next) = self.peek2() {
                        if !next.is_ascii_digit() { break; }
                    }
                }
                s.push(c);
                self.advance();
            } else { break; }
        }
        s.parse::<f64>().map_err(|_| format!("Angka tidak valid: {}", s))
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else { break; }
        }
        s
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            while let Some(c) = self.peek() {
                if c == ' ' || c == '\t' || c == '\r' { self.advance(); } else { break; }
            }

            let c = match self.peek() {
                Some(c) => c,
                None => { tokens.push(Token::Eof); break; }
            };

            let token = match c {
                '\n' => { self.advance(); Token::Newline }
                '%' => {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' { break; }
                        self.advance();
                    }
                    continue; 
                }
                '0'..='9' => {
                    let n = self.read_number()?;
                    Token::Number(n)
                }
                '.' => {
                    if let Some(next) = self.peek2() {
                        match next {
                            '*' => { self.advance(); self.advance(); Token::DotStar }
                            '/' => { self.advance(); self.advance(); Token::DotSlash }
                            '^' => { self.advance(); self.advance(); Token::DotCaret }
                            '0'..='9' => {
                                let n = self.read_number()?;
                                Token::Number(n)
                            }
                            _ => return Err(format!("Karakter '.' tidak valid")),
                        }
                    } else { return Err("Karakter '.' di akhir input".to_string()); }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "for" => Token::For,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "end" => Token::End,
                        _ => Token::Identifier(ident),
                    }
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); Token::Eq }
                    else { Token::Assign }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); Token::Neq }
                    else { return Err("Karakter '!' harus diikuti '='".to_string()); }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); Token::Le }
                    else { Token::Lt }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); Token::Ge }
                    else { Token::Gt }
                }
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '[' => { self.advance(); Token::LBracket }
                ']' => { self.advance(); Token::RBracket }
                ',' => { self.advance(); Token::Comma }
                ';' => { self.advance(); Token::Semicolon }
                ':' => { self.advance(); Token::Colon }
                '\'' => {
                    self.advance();
                    // PERBAIKAN BUG 1: Cek apakah ini String atau Transpose
                    let is_string = if let Some(next) = self.peek() {
                        next.is_alphanumeric() || next == ' '
                    } else { false };

                    if is_string {
                        let mut s = String::new();
                        while let Some(ch) = self.peek() {
                            if ch == '\'' {
                                self.advance();
                                break;
                            }
                            s.push(ch);
                            self.advance();
                        }
                        Token::String(s)
                    } else {
                        Token::Transpose
                    }
                }
                '^' => { self.advance(); Token::Caret }
                _ => return Err(format!("Karakter tidak dikenal: '{}'", c)),
            };
            tokens.push(token);
        }
        Ok(tokens)
    }
}
