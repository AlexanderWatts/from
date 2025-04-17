use std::cell::Cell;
use token::Token;

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
        self.scan()
    }

    fn scan(&self) -> Token {
        while let Some(' ' | '\n' | '\t' | '\r') = self.peek_char() {
            self.next_char();
        }

        self.start.set(self.current.get());

        match self.next_char() {
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('=') => Token::Equal,
            Some(';') => Token::Semicolon,
            Some('@') => {
                while let Some('a'..='z') = self.peek_char() {
                    self.next_char();
                }

                match self.input.get(self.start.get() + 1..self.current.get()) {
                    Some(word) => match word.into_iter().collect::<String>().as_str() {
                        _ => return Token::Attribute(word.into_iter().collect()),
                    },
                    _ => return Token::Error,
                }
            }
            Some('\"') => {
                while let Some(char @ '\u{0000}'..='\u{10FFFF}') = self.peek_char() {
                    if matches!(char, '\"') {
                        break;
                    }

                    self.next_char();
                }

                if !matches!(self.peek_char(), Some('\"')) {
                    return Token::Error;
                }

                self.next_char();

                match self.input.get(self.start.get()..self.current.get()) {
                    Some(value) => Token::Literal(value.into_iter().collect()),
                    _ => Token::Error,
                }
            }
            Some('a'..='z' | 'A'..='Z') => {
                while let Some('a'..='z' | 'A'..='Z') = self.peek_char() {
                    self.next_char();
                }

                match self.input.get(self.start.get()..self.current.get()) {
                    Some(word) => Token::Identifier(word.into_iter().collect()),
                    _ => return Token::Error,
                }
            }
            Some(_) => Token::Error,
            None => Token::End,
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
    fn tokenize_attributes() {
        let lexer = Lexer::new(r#"@action="/api";"#);

        assert_eq!(Token::Attribute("action".to_string()), lexer.token());
        assert_eq!(Token::Equal, lexer.token());
        assert_eq!(Token::Literal("\"/api\"".to_string()), lexer.token());
        assert_eq!(Token::Semicolon, lexer.token());
    }

    #[test]
    fn tokenize_strings() {
        let lexer = Lexer::new(r#"div {"Hello, 🌎!"}"#);

        assert_eq!(Token::Identifier("div".to_string()), lexer.token());
        assert_eq!(Token::LeftBrace, lexer.token());
        assert_eq!(Token::Literal("\"Hello, 🌎!\"".to_string()), lexer.token());
        assert_eq!(Token::RightBrace, lexer.token());
        assert_eq!(Token::End, lexer.token());
    }

    #[test]
    fn tokenize_input() {
        let lexer = Lexer::new("div {}");

        assert_eq!(Token::Identifier("div".to_string()), lexer.token());
        assert_eq!(Token::LeftBrace, lexer.token());
        assert_eq!(Token::RightBrace, lexer.token());
        assert_eq!(Token::End, lexer.token());
    }

    #[test]
    fn identify_core_characters() {
        assert_eq!(Token::LeftBrace, Lexer::new("{").scan());
        assert_eq!(Token::RightBrace, Lexer::new("}").scan());
    }

    #[test]
    fn identify_keywords() {
        assert_eq!(
            Token::Identifier("div".to_string()),
            Lexer::new("div").scan()
        );
        assert_eq!(
            Token::Identifier("span".to_string()),
            Lexer::new("span").scan()
        );
        assert_eq!(
            Token::Identifier("something".to_string()),
            Lexer::new("something").scan()
        );
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
