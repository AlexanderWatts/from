use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    P,
    Form,
    Input,
    Button,
    Div,
    Span,
    Literal,
    Attribute,
    Equal,
    LeftBrace,
    RightBrace,
    Error,
    End,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P => write!(f, "\"p\""),
            Self::Form => write!(f, "\"form\""),
            Self::Input => write!(f, "\"input\""),
            Self::Button => write!(f, "\"button\""),
            Self::Div => write!(f, "\"div\""),
            Self::Span => write!(f, "\"span\""),
            Self::Attribute => write!(f, "attribute"),
            Self::Literal => write!(f, "literal"),
            Self::Equal => write!(f, "="),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Error => write!(f, "error"),
            Self::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    P,
    Form,
    Input,
    Button,
    Div,
    Span,
    Attribute(String),
    Literal(String),
    Equal,
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
            Token::Attribute(_) => TokenType::Attribute,
            Token::Equal => TokenType::Equal,
            Token::Literal(_) => TokenType::Literal,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Error => TokenType::Error,
            Token::End => TokenType::End,
            Token::P => TokenType::P,
            Token::Form => TokenType::Form,
            Token::Input => TokenType::Input,
            Token::Button => TokenType::Button,
        }
    }
}
