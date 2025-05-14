use super::token::Token;

pub struct Lexer {
    /// String to analyze
    input: Vec<char>,

    /// current position in input (points to current char)
    position: usize,

    /// current reading position in input (after current char)
    next_position: usize,

    /// current char under examination
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let mut l = Self {
            input,
            position: 0,
            next_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    /// get and analyze next token, returning the correct TokenType
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let t = match self.ch {
            None => Token::EOF,
            Some(c) => match c {
                '[' => Token::LSquare,
                ']' => Token::RSquare,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ',' => Token::Comma,
                ':' => Token::Colon,
                'n' => self.read_null(),
                '"' => {
                    let text = self.read_string();
                    Token::StringLiteral(text)
                }
                _ => Token::Illegal(c),
            },
        };
        self.read_char();
        t
    }

    fn read_char(&mut self) {
        self.ch = self.peek();
        if self.ch.is_some() {
            self.position = self.next_position;
            self.next_position += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.next_position).copied()
    }

    fn read_string(&mut self) -> String {
        // TODO: obtener las comillas escapadas y no asumir que son el final del string
        self.read_char();
        let start_position = self.position;
        while self.ch != Some('"') && self.ch.is_some() {
            self.read_char();
        }
        let end_position = self.position;
        self.input[start_position..end_position]
            .iter()
            .collect::<String>()
    }

    fn read_null(&mut self) -> Token {
        // TODO: detect when the null is the last keyword, in other words, when the object finish
        let start_position = self.position;
        let mut end_position = self.position;
        while (self.peek() != Some(',') || self.peek() != Some('}')) && self.ch.is_some() {
            self.read_char();
            end_position = self.position;
            self.skip_whitespace();
        }
        let keyword = self.input[start_position..end_position + 1]
            .iter()
            .collect::<String>();
        if keyword == "null".to_string() {
            return Token::NullLiteral;
        }
        Token::InvalidKeyword(keyword)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }
}
