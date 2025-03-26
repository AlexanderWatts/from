use proto::{Attribute, Element, Proto};
use token::{Token, TokenType};
use token_buffer::TokenBuffer;
mod token_buffer;

///
/// Grammar
///
/// program := element
/// element := ('div'
///     | 'span'
///     | 'p'
///     | 'form'
///     | 'input'
///     | 'button'
/// ) element_block | LITERAL
/// element_block := '{' (element | attribute)* '}'
/// attribute := '@' LITERAL '=' LITERAL ';'
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

    pub fn parse(&mut self) -> Result<Proto, ()> {
        self.element()
    }

    fn element(&mut self) -> Result<Proto, ()> {
        let token = &self.next_or_err([
            TokenType::Literal,
            TokenType::Div,
            TokenType::Span,
            TokenType::Button,
            TokenType::P,
            TokenType::Input,
            TokenType::Form,
        ])?;
        let element_type = TokenType::from(token);

        Ok(match token {
            Token::Literal(literal) => Proto::Literal(literal.to_string()),
            _ => {
                let block = self.element_block()?;
                Proto::Element(Element::new(&element_type.to_string(), block))
            }
        })
    }

    fn element_block(&mut self) -> Result<Vec<Proto>, ()> {
        self.next_or_err([TokenType::LeftBrace])?;

        let mut elements: Vec<Proto> = vec![];

        while !matches!(self.token_buffer.peek_token_type(), TokenType::RightBrace) {
            match self.token_buffer.peek() {
                Token::Attribute(_) => elements.push(self.attribute()?),
                _ => elements.push(self.element()?),
            }
        }

        self.next_or_err([TokenType::RightBrace])?;

        Ok(elements)
    }

    fn attribute(&mut self) -> Result<Proto, ()> {
        let name = match self.next_or_err([TokenType::Attribute])? {
            Token::Attribute(name) => name,
            _ => return Err(()),
        };

        self.next_or_err([TokenType::Equal])?;

        let value = match self.next_or_err([TokenType::Literal])? {
            Token::Literal(literal) => literal,
            _ => return Err(()),
        };

        Ok(Proto::Attribute(Attribute::new(&name, &value)))
    }

    fn next_or_err<T>(&mut self, expected_token_types: T) -> Result<Token, ()>
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
            Ok(Token::Attribute("style".to_string())),
            Parser::new(r#"@style="color: red;""#).next_or_err([TokenType::Attribute])
        );

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
    fn parse_elements_with_attributes() {
        assert_eq!(
            Ok(Proto::Element(Element::new(
                "\"span\"",
                vec![
                    Proto::Attribute(Attribute::new("style", "\"\"")),
                    Proto::Element(Element::new("\"div\"", vec![]))
                ]
            ))),
            Parser::new(r#"span {@style="" div{}}"#).parse(),
        );

        assert_eq!(
            Ok(Proto::Element(Element::new(
                "\"span\"",
                vec![Proto::Attribute(Attribute::new("style", "\"\""))]
            ))),
            Parser::new(r#"span {@style=""}"#).parse(),
        );
    }

    #[test]
    fn parse_elements() {
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
