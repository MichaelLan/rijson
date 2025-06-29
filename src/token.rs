use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    //  structural characters
    LBrace,  // {
    RBrace,  // }
    LSquare, // [
    RSquare, // ]
    Comma,   // ,
    Colon,   // :

    // Literals
    StringLiteral(String), // strings
    NumberLiteral(String), // 1, 21.5, 2e10
    BooleanLiteral(bool),  // true, false
    NullLiteral,           // null

    EOF,
    Illegal(char),
    InvalidKeyword(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LSquare => write!(f, "["),
            Self::RSquare => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::StringLiteral(s) => write!(f, "\"{s}\""),
            Self::NumberLiteral(n) => write!(f, "{n}"),
            Self::BooleanLiteral(b) => write!(f, "{b}"),
            Self::NullLiteral => write!(f, "null"),
            Self::Illegal(c) => write!(f, "{c}"),
            Self::InvalidKeyword(key) => write!(f, "{key}"),
            Self::EOF => write!(f, ""),
        }
    }
}
