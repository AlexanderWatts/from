#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
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
                input: vec!['d', 'i', 'v', ' ', '{', '}']
            },
            Lexer::new("div {}")
        );
    }
}
