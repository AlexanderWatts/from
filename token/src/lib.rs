use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Div,
    Span,
    Literal,
    LeftBrace,
    RightBrace,
    Error,
    End,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Div => write!(f, "\"div\""),
            Self::Span => write!(f, "\"span\""),
            Self::Literal => write!(f, "literal"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Error => write!(f, "error"),
            Self::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Div,
    Span,
    Literal(String),
    LeftBrace,
    RightBrace,
    Error,
    End,
}

impl From<&Token> for TokenType {
    fn from(token: &Token) -> Self {
        match token {
            Token::Div => TokenType::Div,
            Token::Span => TokenType::Span,
            Token::Literal(_) => TokenType::Literal,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Error => TokenType::Error,
            Token::End => TokenType::End,
        }
    }
}
