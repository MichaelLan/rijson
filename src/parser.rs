use std::{collections::HashMap, fmt::Display};

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
    pub lexer: Lexer,
    started: bool,
    finished: bool,
}

impl Iterator for Parser {
    type Item = Result<JsonValue, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.started {
            match self.lexer.next_token() {
                Token::LSquare => {
                    self.started = true;
                }
                Token::EOF => {
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
            match self.lexer.next_token() {
                Token::LBrace => match self.parse_object() {
                    Ok(obj) => return Some(Ok(obj)),
                    Err(e) => {
                        self.finished = true;
                        return Some(Err(e));
                    }
                },
                Token::RSquare => {
                    self.finished = true;
                    return None;
                }
                Token::Comma => {
                    continue;
                }
                Token::EOF => {
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
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            started: false,
            finished: false,
        }
    }

    pub fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut obj: HashMap<String, JsonValue> = HashMap::new();
        let mut is_key: bool = true;
        let mut key: String = "".into();
        let mut value: Option<JsonValue> = None;

        loop {
            match self.lexer.next_token() {
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
                Token::LSquare => return Err("Arrays inside objects not implemented".into()),
                Token::RSquare => return Err("Unexpected closing square bracket in object".into()),
                Token::Illegal(c) => return Err(format!("Ilegal character, {c}")),
                Token::InvalidKeyword(key) => return Err(format!("invalid keyword {key}")),
                Token::EOF => return Err("Unexpected end of input while parsing object".into()),
            }
        }

        Ok(JsonValue::Object(obj))
    }
}
