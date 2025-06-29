use std::{collections::HashMap, fmt::Display, iter::Peekable};

use crate::{lexer::Lexer, token::Token};

pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Null => write!(f, "null"),
            Self::Object(obj) => {
                write!(f, "{{")?;

                let mut first = true;
                for (k, v) in obj {
                    if !first {
                        write!(f, ", ")?;
                    }

                    write!(f, "\"{}\": {}", k, v)?;
                    first = false;
                }

                write!(f, "}}")
            }
            Self::Array(arr) => {
                write!(f, "[")?;

                let mut first = true;
                for value in arr {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                    first = false;
                }

                write!(f, "]")
            }
        }
    }
}

pub struct Parser {
    pub lexer: Peekable<Lexer>,
    started: bool,
    finished: bool,
}

// implemented only for array of objects
impl Iterator for Parser {
    type Item = Result<JsonValue, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.started {
            match self.lexer.next() {
                Some(Token::LSquare) => {
                    self.started = true;
                }
                None => {
                    self.finished = true;
                    return None;
                }
                _ => {
                    self.finished = true;
                    return Some(Err("Expected array start".into()));
                }
            }
        }

        loop {
            match self.lexer.next() {
                Some(Token::LBrace) => match self.parse_object() {
                    Ok(obj) => return Some(Ok(obj)),
                    Err(e) => {
                        self.finished = true;
                        return Some(Err(e));
                    }
                },
                Some(Token::RSquare) => {
                    self.finished = true;
                    return None;
                }
                Some(Token::Comma) => {
                    continue;
                }
                None => {
                    self.finished = true;
                    return None;
                }
                _ => {
                    self.finished = true;
                    return Some(Err("unexpected token in array".into()));
                }
            }
        }
    }
}

impl Parser {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            started: false,
            finished: false,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        if let Some(token_to_parse) = self.lexer.next() {
            match token_to_parse {
                Token::LSquare => return self.parse_array(),
                Token::LBrace => return self.parse_object(),
                _ => return Err(format!("unexpected character {token_to_parse}")),
            }
        }
        Err("invaled EOF".into())
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut arr: Vec<JsonValue> = vec![];

        loop {
            if let Some(token_to_parse) = self.lexer.next() {
                match token_to_parse {
                    Token::StringLiteral(s) => {
                        if !matches!(self.lexer.peek(), Some(Token::Comma) | Some(Token::RSquare)) {
                            return Err("invalid token in array number".into());
                        }
                        arr.push(JsonValue::String(s));
                    }
                    Token::NumberLiteral(n) => {
                        if !matches!(self.lexer.peek(), Some(Token::Comma) | Some(Token::RSquare)) {
                            return Err("invalid token in array number".into());
                        }

                        arr.push(JsonValue::Number(n));
                    }
                    Token::BooleanLiteral(b) => {
                        if !matches!(self.lexer.peek(), Some(Token::Comma) | Some(Token::RSquare)) {
                            return Err("invalid token in array number".into());
                        }
                        arr.push(JsonValue::Boolean(b));
                    }
                    Token::NullLiteral => {
                        if !matches!(self.lexer.peek(), Some(Token::Comma) | Some(Token::RSquare)) {
                            return Err("invalid token in array number".into());
                        }
                        arr.push(JsonValue::Null);
                    }
                    Token::LBrace => {
                        let obj = self.parse_object()?;
                        arr.push(obj);
                    }
                    Token::RBrace => return Err("invalid character }".into()),
                    Token::LSquare => {
                        let arr2 = self.parse_array()?;
                        arr.push(arr2);
                    }
                    Token::RSquare => {
                        // TODO: Debo hacer algo mas?
                        break;
                    }
                    Token::Comma => {
                        // TODO: Debo validar que el siguiente sea un token valido?
                        continue;
                    }
                    Token::Colon => return Err("illegal chararacter, colon in array".into()),
                    Token::Illegal(c) => return Err(format!("illegal chararacter {c}")),
                    Token::InvalidKeyword(key) => return Err(format!("invalid keyword {key}")),
                    Token::EOF => return Err("unexpected EOF".into()),
                }
            }
        }

        Ok(JsonValue::Array(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut obj: HashMap<String, JsonValue> = HashMap::new();
        let mut is_key: bool = true;
        let mut key: String = "".into();
        let mut value: Option<JsonValue> = None;

        loop {
            if let Some(token_to_parse) = self.lexer.next() {
                match token_to_parse {
                    Token::StringLiteral(s) => {
                        if is_key {
                            key = s;
                        } else {
                            value = Some(JsonValue::String(s));
                        }
                    }
                    Token::NullLiteral => value = Some(JsonValue::Null),
                    Token::NumberLiteral(n) => value = Some(JsonValue::Number(n)),
                    Token::BooleanLiteral(b) => value = Some(JsonValue::Boolean(b)),
                    Token::Colon => {
                        is_key = false;
                    }
                    Token::Comma => {
                        if key.is_empty() || value.is_none() {
                            return Err("empty key or value in comma statement".into());
                        }
                        let _ = obj.insert(key, value.unwrap());
                        key = "".into();
                        value = None;
                        is_key = true;
                    }
                    Token::LBrace => {
                        let objv2 = self.parse_object()?;
                        value = Some(objv2);
                    }
                    Token::RBrace => {
                        if key.is_empty() || value.is_none() {
                            return Err("empty key or value in RBrace statement".into());
                        }
                        let _ = obj.insert(key, value.unwrap());
                        break;
                    }
                    Token::LSquare => {
                        let arr = self.parse_array()?;
                        value = Some(arr)
                    }
                    Token::RSquare => {
                        return Err("Unexpected closing square bracket in object".into())
                    }
                    Token::Illegal(c) => return Err(format!("Ilegal character, {c}")),
                    Token::InvalidKeyword(key) => return Err(format!("invalid keyword {key}")),
                    Token::EOF => {
                        return Err("Unexpected end of input while parsing object".into());
                    }
                }
            }
        }

        Ok(JsonValue::Object(obj))
    }
}
