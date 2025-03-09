#[derive(Debug, PartialEq)]
pub enum TokenType {
    Div,
    LeftBrace,
    RightBrace,
    Error,
    End,
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
