#[derive(Debug, PartialEq)]
pub enum TokenType {
    Equal,
    Semicolon,
    LeftBrace,
    RightBrace,
    Identifier,
    Literal,
    Attribute,
    Let,
    Error,
    End,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
}

impl TryFrom<&str> for Keyword {
    type Error = Token;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "let" => Ok(Keyword::Let),
            _ => Err(Token::Error),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Attribute(String),
    Literal(String),
    Identifier(String),
    Keyword(Keyword),
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
            Token::Keyword(keyword) => match keyword {
                Keyword::Let => TokenType::Let,
            },
            Token::Equal => TokenType::Equal,
            Token::Literal(_) => TokenType::Literal,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Error => TokenType::Error,
            Token::End => TokenType::End,
        }
    }
}
