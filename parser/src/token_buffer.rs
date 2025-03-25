use lexer::Lexer;
use std::mem::replace;
use token::{Token, TokenType};

#[derive(Debug)]
pub struct TokenBuffer {
    lexer: Lexer,
    token: Token,
}

impl TokenBuffer {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::new(input);
        let token = lexer.token();

        Self { lexer, token }
    }

    pub fn next(&mut self) -> Token {
        replace(&mut self.token, self.lexer.token())
    }

    pub fn peek(&self) -> &Token {
        &self.token
    }

    pub fn peek_token_type(&self) -> TokenType {
        TokenType::from(&self.token)
    }
}

#[cfg(test)]
mod token_buffer {
    use super::*;

    #[test]
    fn get_and_consume_next_token() {
        let mut token_buffer = TokenBuffer::new("div {}");

        assert_eq!(Token::Div, token_buffer.next());
        assert_eq!(Token::LeftBrace, token_buffer.next());
        assert_eq!(Token::RightBrace, token_buffer.next());
        assert_eq!(Token::End, token_buffer.next());
    }

    #[test]
    fn peek_current_token() {
        let token_buffer = TokenBuffer::new("div {}");

        assert_eq!(&Token::Div, token_buffer.peek());
        assert_eq!(&Token::Div, token_buffer.peek());
    }

    #[test]
    fn new() {
        assert_eq!(Token::End, TokenBuffer::new("").token);
    }
}
