#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,      // =
    LParen,
    RParen,
    Eof,
}

impl Token {
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        match self {
            Token::Number(n) => format!("Number({})", n),
            Token::Identifier(s) => format!("Identifier({})", s),
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Star => "Star".to_string(),
            Token::Slash => "Slash".to_string(),
            Token::Assign => "Assign".to_string(),
            Token::LParen => "LParen".to_string(),
            Token::RParen => "RParen".to_string(),
            Token::Eof => "Eof".to_string(),
        }
    }
}
