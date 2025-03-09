use std::cell::Cell;

use token::TokenType;

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

    fn scan(&self) -> TokenType {
        self.start.set(self.current.get());

        match self.next_char() {
            Some('a'..'z' | 'A'..'Z') => {
                while let Some('a'..'z' | 'A'..'Z') = self.peek_char() {
                    self.next_char();
                }

                match self.input.get(self.start.get()..self.current.get()) {
                    Some(word) => match word.into_iter().collect::<String>().as_str() {
                        "div" => TokenType::Div,
                        "span" => TokenType::Span,
                        _ => return TokenType::Error,
                    },
                    _ => return TokenType::Error,
                }
            }
            _ => TokenType::Error,
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
