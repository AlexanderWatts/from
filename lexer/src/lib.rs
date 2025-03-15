use std::cell::Cell;

use token::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    start: Cell<usize>,
    current: Cell<usize>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            start: Cell::new(0),
            current: Cell::new(0),
        }
    }

    pub fn token(&self) -> Token {
        let token_type = self.scan();

        Token::new(token_type)
    }

    fn scan(&self) -> TokenType {
        while let Some(' ') = self.peek_char() {
            self.next_char();
        }

        self.start.set(self.current.get());

        match self.next_char() {
            Some('{') => TokenType::LeftBrace,
            Some('}') => TokenType::RightBrace,
            Some('a'..='z' | 'A'..='Z') => {
                while let Some('a'..='z' | 'A'..='Z') = self.peek_char() {
                    self.next_char();
                }

                match self.input.get(self.start.get()..self.current.get()) {
                    Some(word) => match word.into_iter().collect::<String>().as_str() {
                        "div" => TokenType::Div,
                        "span" => TokenType::Span,
                        "return" => TokenType::Return,
                        _ => return TokenType::Error,
                    },
                    _ => return TokenType::Error,
                }
            }
            Some(_) => TokenType::Error,
            None => TokenType::End,
        }
    }

    fn next_char(&self) -> Option<&char> {
        let next_char = self.input.get(self.current.get());

        self.current.set(self.current.get() + 1);

        next_char
    }

    fn peek_char(&self) -> Option<&char> {
        self.input.get(self.current.get())
    }
}

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn tokenize_input() {
        let lexer = Lexer::new("div {}");

        assert_eq!(Token::new(TokenType::Div), lexer.token());
        assert_eq!(Token::new(TokenType::LeftBrace), lexer.token());
        assert_eq!(Token::new(TokenType::RightBrace), lexer.token());
        assert_eq!(Token::new(TokenType::End), lexer.token());
    }

    #[test]
    fn identify_core_characters() {
        assert_eq!(TokenType::LeftBrace, Lexer::new("{").scan());
        assert_eq!(TokenType::RightBrace, Lexer::new("}").scan());
    }

    #[test]
    fn identify_keywords() {
        assert_eq!(TokenType::Div, Lexer::new("div").scan());
        assert_eq!(TokenType::Span, Lexer::new("span").scan());
        assert_eq!(TokenType::Error, Lexer::new("something").scan());
    }

    #[test]
    fn peek_char() {
        let lexer = Lexer::new("div {}");

        assert_eq!(Some(&'d'), lexer.peek_char());
        assert_eq!(Some(&'d'), lexer.peek_char());
    }

    #[test]
    fn consume_next_char() {
        assert_eq!(Some(&'d'), Lexer::new("div {}").next_char());
    }

    #[test]
    fn new() {
        assert_eq!(
            Lexer {
                input: vec!['d', 'i', 'v', ' ', '{', '}'],
                start: Cell::new(0),
                current: Cell::new(0),
            },
            Lexer::new("div {}")
        );
    }
}
