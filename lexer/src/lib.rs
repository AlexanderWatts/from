use std::cell::Cell;

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
