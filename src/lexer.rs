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
                'n' => self.read_keyword("null", Token::NullLiteral),
                't' => self.read_keyword("true", Token::BooleanLiteral(true)),
                'f' => self.read_keyword("false", Token::BooleanLiteral(false)),
                'a'..='z' | 'A'..='Z' => self.read_identifier_from(self.position),
                '0'..='9' | '-' => self.read_number(),
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

    fn read_number(&mut self) -> Token {
        let start_position = self.position;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            self.read_char();
            // read the fractional part, if any.
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.read_char();
                } else {
                    break;
                }
            }
        }

        // read the exponent, if any.
        if self.peek() == Some('e') || self.peek() == Some('E') {
            self.read_char();

            if self.peek() == Some('+') || self.peek() == Some('-') {
                self.read_char();
            }

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.read_char();
                } else {
                    break;
                }
            }
        }

        let end_position = self.position;
        let identifier = self.input[start_position..=end_position].iter().collect();
        Token::NumberLiteral(identifier)
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let mut result = String::new();

        while let Some(c) = self.ch {
            match c {
                '"' => break,
                '\\' => {
                    self.read_char();
                    match self.ch {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('n') => result.push('\n'), // \n -> newline
                        Some('t') => result.push('\t'), // \t -> tab
                        Some('r') => result.push('\r'), // \r -> carriage return
                        Some('f') => result.push('\u{000C}'), // \f -> form feed
                        Some('b') => result.push('\u{0008}'), // \b -> backspace
                        Some('/') => result.push('/'),  // \/ -> /
                        Some(other) => {
                            result.push('\\');
                            result.push(other);
                        }
                        None => break,
                    }
                }
                other => result.push(other),
            }
            self.read_char();
        }
        result
    }

    fn read_char_is_matched(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek() {
            if c == expected {
                self.read_char();
                return true;
            }
        }
        false
    }

    fn read_identifier_from(&mut self, start_position: usize) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }

        let identifier = self.input[start_position..=self.position]
            .iter()
            .collect::<String>();

        Token::InvalidKeyword(identifier)
    }

    fn validate_keyword(&mut self, keyword: &str) -> bool {
        for c in keyword[1..].chars() {
            if !self.read_char_is_matched(c) {
                return false;
            }
        }
        true
    }

    fn read_keyword(&mut self, keyword: &str, token: Token) -> Token {
        let start_position = self.position;

        if self.validate_keyword(keyword) {
            if let Some(next_char) = self.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    self.read_identifier_from(start_position)
                } else {
                    token
                }
            } else {
                token
            }
        } else {
            self.read_identifier_from(start_position)
        }
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
