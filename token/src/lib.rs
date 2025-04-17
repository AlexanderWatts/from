#[derive(Debug, PartialEq)]
pub enum TokenType {
    Equal,
    Semicolon,
    LeftBrace,
    RightBrace,
    Identifier,
    Literal,
    Attribute,
    Error,
    End,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Attribute(String),
    Literal(String),
    Identifier(String),
    Semicolon,
    Equal,
    LeftBrace,
    RightBrace,
    Error,
    End,
}

impl From<&Token> for TokenType {
    fn from(token: &Token) -> Self {
        match token {
            Token::Attribute(_) => TokenType::Attribute,
            Token::Identifier(_) => TokenType::Identifier,
            Token::Semicolon => TokenType::Semicolon,
            Token::Equal => TokenType::Equal,
            Token::Literal(_) => TokenType::Literal,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Error => TokenType::Error,
            Token::End => TokenType::End,
        }
    }
}
