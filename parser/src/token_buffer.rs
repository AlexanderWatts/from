use std::cell::RefCell;

use lexer::Lexer;
use token::Token;

pub struct TokenBuffer {
    lexer: Lexer,
    token: RefCell<Token>,
}

impl TokenBuffer {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::new(input);
        let token = RefCell::new(lexer.token());

        Self { lexer, token }
    }
}

#[cfg(test)]
mod token_buffer {
    use token::TokenType;

    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Token::new(TokenType::End),
            *TokenBuffer::new("").token.borrow()
        );
    }
}
