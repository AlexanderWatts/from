#[derive(Debug)]
pub enum TokenType {
    Div,
    LeftBrace,
    RightBrace,
    Error,
    End,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(&self, token_type: TokenType) -> Self {
        Self { token_type }
    }
}
