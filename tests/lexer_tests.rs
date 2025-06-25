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
        Token::StringLiteral("falsee\"a\" gol".to_string()),
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
        Token::StringLiteral("complex\"".to_string()),
        Token::RSquare,
        Token::Comma,
        Token::StringLiteral("data".to_string()),
        Token::Colon,
        Token::NullLiteral,
        Token::Comma,
        Token::StringLiteral("description".to_string()),
        Token::Colon,
        Token::StringLiteral("This is a string with \"quotes\" and\nnewlines.".to_string()),
        Token::Comma,
        Token::StringLiteral("path".to_string()),
        Token::Colon,
        Token::StringLiteral("C:\\Users\\Test".to_string()),
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
      "age": 1,
      "fail": 1.212345678,
      "number": -2e-10
    }
    "#,
    );

    let expected = vec![
        Token::LBrace,
        Token::StringLiteral("age".to_string()),
        Token::Colon,
        Token::NumberLiteral("1".to_string()),
        Token::Comma,
        Token::StringLiteral("fail".to_string()),
        Token::Colon,
        Token::NumberLiteral("1.212345678".to_string()),
        Token::Comma,
        Token::StringLiteral("number".to_string()),
        Token::Colon,
        Token::NumberLiteral("-2e-10".to_string()),
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
fn test_number_comprehensive() {
    let input = String::from(
        r#"
    {
        "integer": 42,
        "negative": -17,
        "zero": 0,
        "decimal": 3.14159,
        "negative_decimal": -2.718,
        "scientific_positive": 1.23e4,
        "scientific_negative": 2.5e-3,
        "scientific_uppercase": 1E6,
        "scientific_with_plus": 6.02e+23,
        "very_small": 1e-100,
        "very_large": 9.999e99,
        "zero_decimal": 0.0,
        "leading_zero_decimal": 0.123
    }
    "#,
    );

    let expected = vec![
        Token::LBrace,
        // "integer": 42
        Token::StringLiteral("integer".to_string()),
        Token::Colon,
        Token::NumberLiteral("42".to_string()),
        Token::Comma,
        // "negative": -17
        Token::StringLiteral("negative".to_string()),
        Token::Colon,
        Token::NumberLiteral("-17".to_string()),
        Token::Comma,
        // "zero": 0
        Token::StringLiteral("zero".to_string()),
        Token::Colon,
        Token::NumberLiteral("0".to_string()),
        Token::Comma,
        // "decimal": 3.14159
        Token::StringLiteral("decimal".to_string()),
        Token::Colon,
        Token::NumberLiteral("3.14159".to_string()),
        Token::Comma,
        // "negative_decimal": -2.718
        Token::StringLiteral("negative_decimal".to_string()),
        Token::Colon,
        Token::NumberLiteral("-2.718".to_string()),
        Token::Comma,
        // "scientific_positive": 1.23e4
        Token::StringLiteral("scientific_positive".to_string()),
        Token::Colon,
        Token::NumberLiteral("1.23e4".to_string()),
        Token::Comma,
        // "scientific_negative": 2.5e-3
        Token::StringLiteral("scientific_negative".to_string()),
        Token::Colon,
        Token::NumberLiteral("2.5e-3".to_string()),
        Token::Comma,
        // "scientific_uppercase": 1E6
        Token::StringLiteral("scientific_uppercase".to_string()),
        Token::Colon,
        Token::NumberLiteral("1E6".to_string()),
        Token::Comma,
        // "scientific_with_plus": 6.02e+23
        Token::StringLiteral("scientific_with_plus".to_string()),
        Token::Colon,
        Token::NumberLiteral("6.02e+23".to_string()),
        Token::Comma,
        // "very_small": 1e-100
        Token::StringLiteral("very_small".to_string()),
        Token::Colon,
        Token::NumberLiteral("1e-100".to_string()),
        Token::Comma,
        // "very_large": 9.999e99
        Token::StringLiteral("very_large".to_string()),
        Token::Colon,
        Token::NumberLiteral("9.999e99".to_string()),
        Token::Comma,
        // "zero_decimal": 0.0
        Token::StringLiteral("zero_decimal".to_string()),
        Token::Colon,
        Token::NumberLiteral("0.0".to_string()),
        Token::Comma,
        // "leading_zero_decimal": 0.123
        Token::StringLiteral("leading_zero_decimal".to_string()),
        Token::Colon,
        Token::NumberLiteral("0.123".to_string()),
        Token::RBrace,
    ];

    let mut l = lexer::Lexer::new(input.chars().collect());
    for (i, expected_token) in expected.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(*expected_token, tok, "Token mismatch at position {}", i);
    }

    let eof = l.next_token();
    assert_eq!(eof, Token::EOF);
}

#[test]
fn test_number_edge_cases() {
    // Array de números para probar casos específicos
    let test_cases = vec![
        ("0", "0"),
        ("42", "42"),
        ("-0", "-0"),
        ("-42", "-42"),
        ("3.14", "3.14"),
        ("-3.14", "-3.14"),
        ("1e10", "1e10"),
        ("1E10", "1E10"),
        ("1e+10", "1e+10"),
        ("1e-10", "1e-10"),
        ("1.5e10", "1.5e10"),
        ("1.5E-10", "1.5E-10"),
        ("0.1", "0.1"),
        ("0.0", "0.0"),
    ];

    for (input, expected) in test_cases {
        let mut lexer = lexer::Lexer::new(input.chars().collect());
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::NumberLiteral(expected.to_string()),
            "Failed for input: {}",
            input
        );

        // Verificar que después viene EOF
        let eof = lexer.next_token();
        assert_eq!(eof, Token::EOF);
    }
}

#[test]
fn test_numbers_in_array() {
    let input = "[1, -2, 3.14, 1e10, -2.5e-3]";
    let expected = vec![
        Token::LSquare,
        Token::NumberLiteral("1".to_string()),
        Token::Comma,
        Token::NumberLiteral("-2".to_string()),
        Token::Comma,
        Token::NumberLiteral("3.14".to_string()),
        Token::Comma,
        Token::NumberLiteral("1e10".to_string()),
        Token::Comma,
        Token::NumberLiteral("-2.5e-3".to_string()),
        Token::RSquare,
    ];

    let mut lexer = lexer::Lexer::new(input.chars().collect());
    for (i, expected_token) in expected.iter().enumerate() {
        let tok = lexer.next_token();
        assert_eq!(*expected_token, tok, "Token mismatch at position {}", i);
    }
}

#[test]
fn test_number_boundaries() {
    let input = r#"{"count":42,"name":"test","active":true}"#;
    let mut lexer = lexer::Lexer::new(input.chars().collect());

    assert_eq!(lexer.next_token(), Token::LBrace);
    assert_eq!(
        lexer.next_token(),
        Token::StringLiteral("count".to_string())
    );
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::NumberLiteral("42".to_string()));
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(lexer.next_token(), Token::StringLiteral("name".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::StringLiteral("test".to_string()));
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(
        lexer.next_token(),
        Token::StringLiteral("active".to_string())
    );
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::BooleanLiteral(true));
    assert_eq!(lexer.next_token(), Token::RBrace);
}
