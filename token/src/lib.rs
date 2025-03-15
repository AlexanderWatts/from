use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Div,
    Span,
    LeftBrace,
    RightBrace,
    Error,
    Return,
    End,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Div => write!(f, "div"),
            Self::Span => write!(f, "span"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Error => write!(f, "error"),
            Self::Return => write!(f, "return"),
            Self::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}
