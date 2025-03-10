use std::cell::{Ref, RefCell};

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

    pub fn next(&self) -> Token {
        self.token.replace(self.lexer.token())
    }

    pub fn peek(&self) -> Ref<'_, Token> {
        self.token.borrow()
    }
}

#[cfg(test)]
mod token_buffer {
    use token::TokenType;

    use super::*;

    #[test]
    fn consume_input() {
        let token_buffer = TokenBuffer::new("div {}");

        assert_eq!(Token::new(TokenType::Div), token_buffer.next());
        assert_eq!(Token::new(TokenType::LeftBrace), token_buffer.next());
        assert_eq!(Token::new(TokenType::RightBrace), token_buffer.next());
        assert_eq!(Token::new(TokenType::End), token_buffer.next());
    }

    #[test]
    fn peek_current_token() {
        let token_buffer = TokenBuffer::new("div {}");

        assert_eq!(Token::new(TokenType::Div), *token_buffer.peek());
        assert_eq!(Token::new(TokenType::Div), *token_buffer.peek());
    }

    #[test]
    fn new() {
        assert_eq!(
            Token::new(TokenType::End),
            *TokenBuffer::new("").token.borrow()
        );
    }
}
