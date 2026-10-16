#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    DotStar,     // .*
    DotSlash,    // ./
    DotCaret,    // .^
    Assign,      // =
    LParen,
    RParen,
    LBracket,    // [
    RBracket,    // ]
    Comma,       // ,
    Semicolon,   // ;
    Transpose,   // '
    Caret,       // ^ (nanti untuk power)
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
            Token::DotStar => "DotStar".to_string(),
            Token::DotSlash => "DotSlash".to_string(),
            Token::DotCaret => "DotCaret".to_string(),
            Token::Assign => "Assign".to_string(),
            Token::LParen => "LParen".to_string(),
            Token::RParen => "RParen".to_string(),
            Token::LBracket => "LBracket".to_string(),
            Token::RBracket => "RBracket".to_string(),
            Token::Comma => "Comma".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Transpose => "Transpose".to_string(),
            Token::Caret => "Caret".to_string(),
            Token::Eof => "Eof".to_string(),
        }
    }
}
