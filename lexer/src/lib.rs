use std::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    index: Cell<usize>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            index: Cell::new(0),
        }
    }
}

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Lexer {
                input: vec!['d', 'i', 'v', ' ', '{', '}'],
                index: Cell::new(0),
            },
            Lexer::new("div {}")
        );
    }
}
