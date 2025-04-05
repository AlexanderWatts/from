use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Equal,
    LeftBrace,
    RightBrace,
    Identifier,
    Literal,
    Attribute,
    Error,
    End,
}

// Remove in future
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attribute => write!(f, "attribute"),
            Self::Literal => write!(f, "literal"),
            Self::Equal => write!(f, "="),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Error => write!(f, "error"),
            Self::End => write!(f, "end"),
            Self::Identifier => write!(f, "identifier"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Attribute(String),
    Literal(String),
    Identifier(String),
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
            Token::Equal => TokenType::Equal,
            Token::Literal(_) => TokenType::Literal,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Error => TokenType::Error,
            Token::End => TokenType::End,
        }
    }
}
