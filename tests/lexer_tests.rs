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
        "country": "Colombia",
        "city": null
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
        Token::StringLiteral("Colombia".to_string()),
        Token::Comma,
        Token::StringLiteral("city".to_string()),
        Token::Colon,
        Token::NullLiteral,
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
