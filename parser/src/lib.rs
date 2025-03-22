use proto::{Element, Proto};
use token::{Token, TokenType};
use token_buffer::TokenBuffer;
mod token_buffer;

///
/// Grammar
///
/// program := element
/// element := ('div' | 'span') element_block | LITERAL
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

    pub fn parse(&self) -> Result<Proto, ()> {
        self.element()
    }

    fn element(&self) -> Result<Proto, ()> {
        let token = &self.next_or_err([TokenType::Literal, TokenType::Div, TokenType::Span])?;
        let element_type = TokenType::from(token);

        Ok(match token {
            Token::Literal(literal) => Proto::Literal(literal.to_string()),
            _ => {
                let block = self.element_block()?;
                Proto::Element(Element::new(&element_type.to_string(), block))
            }
        })
    }

    fn element_block(&self) -> Result<Vec<Proto>, ()> {
        self.next_or_err([TokenType::LeftBrace])?;

        let mut elements: Vec<Proto> = vec![];

        while !matches!(
            TokenType::from(&*self.token_buffer.peek()),
            TokenType::RightBrace
        ) {
            elements.push(self.element()?);
        }

        self.next_or_err([TokenType::RightBrace])?;

        Ok(elements)
    }

    fn next_or_err<T>(&self, expected_token_types: T) -> Result<Token, ()>
    where
        T: IntoIterator<Item = TokenType>,
    {
        let token = self.token_buffer.next();
        let token_type = TokenType::from(&token);

        if expected_token_types
            .into_iter()
            .any(|expected| token_type == expected)
        {
            return Ok(token);
        }

        Err(())
    }
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn expect_token_type() {
        assert_eq!(
            Ok(Token::Literal("\"Hello, World!\"".to_string())),
            Parser::new(r#""Hello, World!""#).next_or_err([TokenType::Literal])
        );

        assert_eq!(
            Ok(Token::Span),
            Parser::new("span {}").next_or_err([TokenType::Span, TokenType::Div])
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            Ok(Proto::Element(Element::new(
                "\"span\"",
                vec![Proto::Element(Element::new("\"div\"", vec![]))]
            ))),
            Parser::new("span{div{}}").parse(),
        );

        assert_eq!(
            Ok(Proto::Element(Element::new("\"div\"", vec![]))),
            Parser::new("div {}").parse()
        );
    }
}
