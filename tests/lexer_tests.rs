use rijson::{lexer, token::Token};

#[test]
fn test_simple_lexer() {
    let input = String::from(r#"[]{},:?"#);
    let expected = vec![
        Token::LSquare,
        Token::RSquare,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Colon,
        Token::Illegal('?'),
    ];
    let mut l = lexer::Lexer::new(input.chars().collect::<Vec<char>>());
    for i in 0..input.len() {
        let tok = l.next_token();
        assert_eq!(expected[i], tok);
    }
    let eof = l.next_token();
    assert_eq!(eof, Token::EOF)
}

#[test]
fn test_empty_string() {
    let input: Vec<char> = vec!['"', '"'];
    let mut l = lexer::Lexer::new(input);
    let mut tok = l.next_token();
    assert_eq!(Token::StringLiteral("".to_string()), tok);
    tok = l.next_token();
    assert_eq!(Token::EOF, tok)
}

#[test]
fn test_string() {
    let input = String::from(
        r#"
    {
        "name": "Michael", 
        "array": [],
        "country": "Colombia[]",
        "address": "",
        "city": null,
        "x": nullable,
        "y": nul,
        "long": true,
        "large": false,
        "true": tru,
        "false": "falsee\"a\" gol"
    }
    "#,
    );
    let expected = vec![
        Token::LBrace,
        Token::StringLiteral("name".to_string()),
        Token::Colon,
        Token::StringLiteral("Michael".to_string()),
        Token::Comma,
        Token::StringLiteral("array".to_string()),
        Token::Colon,
        Token::LSquare,
        Token::RSquare,
        Token::Comma,
        Token::StringLiteral("country".to_string()),
        Token::Colon,
        Token::StringLiteral("Colombia[]".to_string()),
        Token::Comma,
        Token::StringLiteral("address".to_string()),
        Token::Colon,
        Token::StringLiteral("".to_string()),
        Token::Comma,
        Token::StringLiteral("city".to_string()),
        Token::Colon,
        Token::NullLiteral,
        Token::Comma,
        Token::StringLiteral("x".to_string()),
        Token::Colon,
        Token::InvalidKeyword("nullable".to_string()),
        Token::Comma,
        Token::StringLiteral("y".to_string()),
        Token::Colon,
        Token::InvalidKeyword("nul".to_string()),
        Token::Comma,
        Token::StringLiteral("long".to_string()),
        Token::Colon,
        Token::BooleanLiteral(true),
        Token::Comma,
        Token::StringLiteral("large".to_string()),
        Token::Colon,
        Token::BooleanLiteral(false),
        Token::Comma,
        Token::StringLiteral("true".to_string()),
        Token::Colon,
        Token::InvalidKeyword("tru".to_string()),
        Token::Comma,
        Token::StringLiteral("false".to_string()),
        Token::Colon,
        Token::StringLiteral("falsee\\\"a\\\" gol".to_string()),
        Token::RBrace,
    ];
    let mut l = lexer::Lexer::new(input.chars().collect());
    for i in 0..expected.len() {
        let tok = l.next_token();
        assert_eq!(expected[i], tok);
    }
    let eof = l.next_token();
    assert_eq!(eof, Token::EOF)
}

#[test]
fn test_structure_without_numbers() {
    let input = String::from(
        r#"
    {
      "active": true,
      "details": {
        "tags": ["json", "lexer", "test", "complex\""],
        "data": null,
        "description": "This is a string with \"quotes\" and\nnewlines.",
        "path": "C:\\Users\\Test"
      },
      "list": [
        false,
        "item three",
        { "nested_key": "nested_value" }
      ],
      "unclosed_string": "hello, world",
      "unknown_symbol": $,
      "invalid_true": truee,
      "invalid_null": nulll,
      "just_an_identifier": my_variable_name
    }
    "#,
    );

    let expected = vec![
        Token::LBrace,
        Token::StringLiteral("active".to_string()),
        Token::Colon,
        Token::BooleanLiteral(true),
        Token::Comma,
        Token::StringLiteral("details".to_string()),
        Token::Colon,
        Token::LBrace,
        Token::StringLiteral("tags".to_string()),
        Token::Colon,
        Token::LSquare,
        Token::StringLiteral("json".to_string()),
        Token::Comma,
        Token::StringLiteral("lexer".to_string()),
        Token::Comma,
        Token::StringLiteral("test".to_string()),
        Token::Comma,
        Token::StringLiteral("complex\\\"".to_string()),
        Token::RSquare,
        Token::Comma,
        Token::StringLiteral("data".to_string()),
        Token::Colon,
        Token::NullLiteral,
        Token::Comma,
        Token::StringLiteral("description".to_string()),
        Token::Colon,
        Token::StringLiteral("This is a string with \\\"quotes\\\" and\\nnewlines.".to_string()),
        Token::Comma,
        Token::StringLiteral("path".to_string()),
        Token::Colon,
        Token::StringLiteral("C:\\\\Users\\\\Test".to_string()),
        Token::RBrace,
        Token::Comma,
        Token::StringLiteral("list".to_string()),
        Token::Colon,
        Token::LSquare,
        Token::BooleanLiteral(false),
        Token::Comma,
        Token::StringLiteral("item three".to_string()),
        Token::Comma,
        Token::LBrace,
        Token::StringLiteral("nested_key".to_string()),
        Token::Colon,
        Token::StringLiteral("nested_value".to_string()),
        Token::RBrace,
        Token::RSquare,
        Token::Comma,
        Token::StringLiteral("unclosed_string".to_string()),
        Token::Colon,
        Token::StringLiteral("hello, world".to_string()),
        Token::Comma,
        Token::StringLiteral("unknown_symbol".to_string()),
        Token::Colon,
        Token::Illegal('$'),
        Token::Comma,
        Token::StringLiteral("invalid_true".to_string()),
        Token::Colon,
        Token::InvalidKeyword("truee".to_string()),
        Token::Comma,
        Token::StringLiteral("invalid_null".to_string()),
        Token::Colon,
        Token::InvalidKeyword("nulll".to_string()),
        Token::Comma,
        Token::StringLiteral("just_an_identifier".to_string()),
        Token::Colon,
        Token::InvalidKeyword("my_variable_name".to_string()),
        Token::RBrace,
    ];

    let mut l = lexer::Lexer::new(input.chars().collect());
    for i in 0..expected.len() {
        let tok = l.next_token();
        assert_eq!(expected[i], tok);
    }
    let eof = l.next_token();
    assert_eq!(eof, Token::EOF);
}

#[test]
fn test_number() {
    let input = String::from(
        r#"
    {
      "age": 1
    }
    "#,
    );

    let expected = vec![
        Token::LBrace,
        Token::StringLiteral("age".to_string()),
        Token::Colon,
        Token::NumberLiteral(1.0),
        Token::RBrace,
    ];
    let mut l = lexer::Lexer::new(input.chars().collect());
    for i in 0..expected.len() {
        let tok = l.next_token();
        assert_eq!(expected[i], tok);
    }
    let eof = l.next_token();
    assert_eq!(eof, Token::EOF);
}
