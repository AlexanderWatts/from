use lexer::Lexer;
use std::cell::{Ref, RefCell};
use token::Token;

#[derive(Debug)]
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
    use super::*;

    #[test]
    fn get_and_consume_next_token() {
        let token_buffer = TokenBuffer::new("div {}");

        assert_eq!(Token::Div, token_buffer.next());
        assert_eq!(Token::LeftBrace, token_buffer.next());
        assert_eq!(Token::RightBrace, token_buffer.next());
        assert_eq!(Token::End, token_buffer.next());
    }

    #[test]
    fn peek_current_token() {
        let token_buffer = TokenBuffer::new("div {}");

        assert_eq!(Token::Div, *token_buffer.peek());
        assert_eq!(Token::Div, *token_buffer.peek());
    }

    #[test]
    fn new() {
        assert_eq!(Token::End, *TokenBuffer::new("").token.borrow());
    }
}
