use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    current_line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { input: input.chars().collect(), pos: 0, current_line: 1 }
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
                None => { tokens.push(Token { kind: TokenKind::Eof, line: self.current_line }); break; }
            };

            let token_kind = match c {
                '\n' => { 
                    self.advance(); 
                    let tok = Token { kind: TokenKind::Newline, line: self.current_line };
                    self.current_line += 1;
                    tokens.push(tok);
                    continue;
                }
                '%' => {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' { break; }
                        self.advance();
                    }
                    continue; 
                }
                '0'..='9' => {
                    let n = self.read_number()?;
                    TokenKind::Number(n)
                }
                '.' => {
                    if let Some(next) = self.peek2() {
                        match next {
                            '*' => { self.advance(); self.advance(); TokenKind::DotStar }
                            '/' => { self.advance(); self.advance(); TokenKind::DotSlash }
                            '^' => { self.advance(); self.advance(); TokenKind::DotCaret }
                            '0'..='9' => {
                                let n = self.read_number()?;
                                TokenKind::Number(n)
                            }
                            'a'..='z' | 'A'..='Z' | '_' => {
                                self.advance(); 
                                TokenKind::Dot
                            }
                            _ => return Err(format!("Karakter '.' tidak valid")),
                        }
                    } else { return Err("Karakter '.' di akhir input".to_string()); }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "for" => TokenKind::For,
                        "while" => TokenKind::While,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "end" => TokenKind::End,
                        "switch" => TokenKind::Switch,
                        "case" => TokenKind::Case,
                        "otherwise" => TokenKind::Otherwise,
                        "break" => TokenKind::Break,
                        "continue" => TokenKind::Continue,
                        "function" => TokenKind::Function,
                        _ => TokenKind::Identifier(ident),
                    }
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenKind::Eq }
                    else { TokenKind::Assign }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenKind::Neq }
                    else { return Err("Karakter '!' harus diikuti '='".to_string()); }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenKind::Le }
                    else { TokenKind::Lt }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenKind::Ge }
                    else { TokenKind::Gt }
                }
                '&' => {
                    self.advance();
                    if self.peek() == Some('&') { self.advance(); TokenKind::And } 
                    else { return Err("Gunakan '&&' untuk logika AND".to_string()); }
                }
                '|' => {
                    self.advance();
                    if self.peek() == Some('|') { self.advance(); TokenKind::Or } 
                    else { return Err("Gunakan '||' untuk logika OR".to_string()); }
                }
                '~' => { self.advance(); TokenKind::Not }
                '+' => { self.advance(); TokenKind::Plus }
                '-' => { self.advance(); TokenKind::Minus }
                '*' => { self.advance(); TokenKind::Star }
                '/' => { self.advance(); TokenKind::Slash }
                '(' => { self.advance(); TokenKind::LParen }
                ')' => { self.advance(); TokenKind::RParen }
                '[' => { self.advance(); TokenKind::LBracket }
                ']' => { self.advance(); TokenKind::RBracket }
                '{' => { self.advance(); TokenKind::LBrace }
                '}' => { self.advance(); TokenKind::RBrace }
                ',' => { self.advance(); TokenKind::Comma }
                ';' => { self.advance(); TokenKind::Semicolon }
                ':' => { self.advance(); TokenKind::Colon }
                '\'' => {
                    let prev_c = if self.pos > 0 { Some(self.input[self.pos - 1]) } else { None };
                    let is_transpose = match prev_c {
                        Some(c) => c.is_alphanumeric() || c == ')' || c == ']' || c == '_',
                        None => false,
                    };
                    self.advance();
                    if is_transpose {
                        TokenKind::Transpose
                    } else {
                        let mut s = String::new();
                        while let Some(ch) = self.peek() {
                            if ch == '\'' {
                                self.advance();
                                break;
                            }
                            s.push(ch);
                            self.advance();
                        }
                        TokenKind::String(s)
                    }
                }
                '^' => { self.advance(); TokenKind::Caret }
                _ => return Err(format!("Karakter tidak dikenal: '{}'", c)),
            };
            
            tokens.push(Token { kind: token_kind, line: self.current_line });
        }
        Ok(tokens)
    }
}