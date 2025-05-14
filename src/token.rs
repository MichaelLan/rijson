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
    NumberLiteral(f32),    // 1, 21.5, 2e10
    BooleanLiteral(bool),  // true, false
    NullLiteral,           // null

    EOF,
    Illegal(char),
    InvalidKeyword(String),
}
