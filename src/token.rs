#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64), Identifier(String), String(String),
    Plus, Minus, Star, Slash, DotStar, DotSlash, DotCaret,
    Assign, Eq, Neq, Lt, Gt, Le, Ge,
    LParen, RParen, LBracket, RBracket, Comma, Semicolon, Colon,
    Transpose, Caret, Newline,
    For, While, If, Else, End, Switch, Case, Otherwise, Break, Continue,
    Function,
    Eof,
}

impl Token {
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        match self {
            Token::Number(n) => format!("Number({})", n),
            Token::Identifier(s) => format!("Identifier({})", s),
            Token::String(s) => format!("String(\"{}\")", s),
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Star => "Star".to_string(),
            Token::Slash => "Slash".to_string(),
            Token::DotStar => "DotStar".to_string(),
            Token::DotSlash => "DotSlash".to_string(),
            Token::DotCaret => "DotCaret".to_string(),
            Token::Assign => "Assign".to_string(),
            Token::Eq => "Eq".to_string(),
            Token::Neq => "Neq".to_string(),
            Token::Lt => "Lt".to_string(),
            Token::Gt => "Gt".to_string(),
            Token::Le => "Le".to_string(),
            Token::Ge => "Ge".to_string(),
            Token::LParen => "LParen".to_string(),
            Token::RParen => "RParen".to_string(),
            Token::LBracket => "LBracket".to_string(),
            Token::RBracket => "RBracket".to_string(),
            Token::Comma => "Comma".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Colon => "Colon".to_string(),
            Token::Transpose => "Transpose".to_string(),
            Token::Caret => "Caret".to_string(),
            Token::Newline => "Newline".to_string(),
            Token::For => "For".to_string(),
            Token::While => "While".to_string(),
            Token::If => "If".to_string(),
            Token::Else => "Else".to_string(),
            Token::End => "End".to_string(),
            Token::Switch => "Switch".to_string(),
            Token::Case => "Case".to_string(),
            Token::Otherwise => "Otherwise".to_string(),
            Token::Break => "Break".to_string(),
            Token::Continue => "Continue".to_string(),
            Token::Function => "Function".to_string(),
            Token::Eof => "Eof".to_string(),
        }
    }
}