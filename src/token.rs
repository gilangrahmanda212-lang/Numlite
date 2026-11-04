#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64), Identifier(String), String(String),
    Plus, Minus, Star, Slash, DotStar, DotSlash, DotCaret,
    Assign, Eq, Neq, Lt, Gt, Le, Ge,
    And, Or, Not,
    LParen, RParen, LBracket, RBracket, Comma, Semicolon, Colon,
    Transpose, Caret, Newline,
    For, While, If, Else, End, Switch, Case, Otherwise, Break, Continue,
    Function,
    Dot,
    LBrace, RBrace,
    Eof,
}

impl TokenKind {
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        match self {
            TokenKind::Number(n) => format!("Number({})", n),
            TokenKind::Identifier(s) => format!("Identifier({})", s),
            TokenKind::String(s) => format!("String(\"{}\")", s),
            TokenKind::Plus => "Plus".to_string(),
            TokenKind::Minus => "Minus".to_string(),
            TokenKind::Star => "Star".to_string(),
            TokenKind::Slash => "Slash".to_string(),
            TokenKind::DotStar => "DotStar".to_string(),
            TokenKind::DotSlash => "DotSlash".to_string(),
            TokenKind::DotCaret => "DotCaret".to_string(),
            TokenKind::Assign => "Assign".to_string(),
            TokenKind::Eq => "Eq".to_string(),
            TokenKind::Neq => "Neq".to_string(),
            TokenKind::Lt => "Lt".to_string(),
            TokenKind::Gt => "Gt".to_string(),
            TokenKind::Le => "Le".to_string(),
            TokenKind::Ge => "Ge".to_string(),
            TokenKind::And => "And".to_string(),
            TokenKind::Or => "Or".to_string(),
            TokenKind::Not => "Not".to_string(),
            TokenKind::LParen => "LParen".to_string(),
            TokenKind::RParen => "RParen".to_string(),
            TokenKind::LBracket => "LBracket".to_string(),
            TokenKind::RBracket => "RBracket".to_string(),
            TokenKind::Comma => "Comma".to_string(),
            TokenKind::Semicolon => "Semicolon".to_string(),
            TokenKind::Colon => "Colon".to_string(),
            TokenKind::Transpose => "Transpose".to_string(),
            TokenKind::Caret => "Caret".to_string(),
            TokenKind::Newline => "Newline".to_string(),
            TokenKind::For => "For".to_string(),
            TokenKind::While => "While".to_string(),
            TokenKind::If => "If".to_string(),
            TokenKind::Else => "Else".to_string(),
            TokenKind::End => "End".to_string(),
            TokenKind::Switch => "Switch".to_string(),
            TokenKind::Case => "Case".to_string(),
            TokenKind::Otherwise => "Otherwise".to_string(),
            TokenKind::Break => "Break".to_string(),
            TokenKind::Continue => "Continue".to_string(),
            TokenKind::Function => "Function".to_string(),
            TokenKind::Dot => "Dot".to_string(),
            TokenKind::LBrace => "LBrace".to_string(),
            TokenKind::RBrace => "RBrace".to_string(),
            TokenKind::Eof => "Eof".to_string(),
        }
    }
}