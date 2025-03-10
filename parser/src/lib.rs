use ast::{Block, Element};
use token::{Token, TokenType};
use token_buffer::TokenBuffer;
mod ast;
mod token_buffer;

///
/// Grammar
///
/// program := element
/// element := ('div' | 'span') element_block
/// element_block := '{' element* '}'
///
#[derive(Debug)]
pub struct Parser {
    token_buffer: TokenBuffer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            token_buffer: TokenBuffer::new(input),
        }
    }

    pub fn parse(&self) -> Result<Element, ()> {
        self.element()
    }

    fn element(&self) -> Result<Element, ()> {
        self.next_or_err([TokenType::Div, TokenType::Span])?;

        let block = self.element_block()?;

        Ok(Element::new(block))
    }

    fn element_block(&self) -> Result<Block, ()> {
        self.next_or_err([TokenType::LeftBrace])?;

        self.next_or_err([TokenType::RightBrace])?;

        Ok(Block::default())
    }

    fn next_or_err<T>(&self, expected_token_types: T) -> Result<Token, ()>
    where
        T: IntoIterator<Item = TokenType>,
    {
        match self.token_buffer.next() {
            token
                if expected_token_types
                    .into_iter()
                    .any(|expected| expected == token.token_type) =>
            {
                Ok(token)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn expect_token_type() {
        assert_eq!(
            Ok(Token::new(TokenType::Span)),
            Parser::new("span {}").next_or_err([TokenType::Span, TokenType::Div])
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            Ok(Element {
                block: Block { elements: vec![] },
            }),
            Parser::new("div {}").parse()
        );
    }
}
