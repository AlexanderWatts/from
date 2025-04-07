mod parser_error;
use html_tag::HtmlTag;
use id_generator::IdGenerator;
use parser_error::ParserError;
use proto::{Attribute, Element, Literal, Proto};
use token::{Token, TokenType};
use token_buffer::TokenBuffer;
mod html_tag;
mod id_generator;
mod token_buffer;

///
/// Grammar
///
/// program := element
/// element := html_tag '{' (element | attribute)* '}' | LITERAL
/// html_tag := 'div' | 'span' | 'p' | 'form' | 'input' | 'button'
/// attribute := '@' literal '=' literal
/// literal := LITERAL
///
#[derive(Debug)]
pub struct Parser {
    token_buffer: TokenBuffer,
    id_generator: IdGenerator,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            token_buffer: TokenBuffer::new(input),
            id_generator: IdGenerator::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Proto, ParserError> {
        let proto = self.element()?;

        self.next_or_err([TokenType::End])?;

        Ok(proto)
    }

    fn element(&mut self) -> Result<Proto, ParserError> {
        let element_id = self.id_generator.generate();

        let element_type = match &self.next_or_err([TokenType::Literal, TokenType::Identifier])? {
            Token::Literal(literal) => {
                return Ok(Proto::Literal(Literal::new(
                    self.id_generator.generate(),
                    literal,
                )));
            }
            Token::Identifier(identifier) => HtmlTag::try_from(identifier.as_str())?.to_string(),
            _ => return Err(ParserError::UnexpectedToken),
        };

        self.next_or_err([TokenType::LeftBrace])?;

        let mut attributes = vec![];
        let mut children = vec![];

        while !matches!(self.token_buffer.peek_token_type(), TokenType::RightBrace) {
            match self.token_buffer.peek() {
                Token::Attribute(_) => attributes.push(self.attribute()?),
                Token::Literal(_) => children.push(self.literal()?),
                _ => children.push(self.element()?),
            }
        }

        self.next_or_err([TokenType::RightBrace])?;

        Ok(Proto::Element(Element::new(
            element_id,
            &element_type,
            attributes,
            children,
        )))
    }

    fn attribute(&mut self) -> Result<Proto, ParserError> {
        let name = match self.next_or_err([TokenType::Attribute])? {
            Token::Attribute(name) => name,
            _ => return Err(ParserError::UnknownAttribute),
        };

        self.next_or_err([TokenType::Equal])?;

        let value = match self.next_or_err([TokenType::Literal])? {
            Token::Literal(literal) => literal,
            _ => return Err(ParserError::UnknownAttribute),
        };

        Ok(Proto::Attribute(Attribute::new(&name, &value)))
    }

    fn literal(&mut self) -> Result<Proto, ParserError> {
        let value = match self.next_or_err([TokenType::Literal])? {
            Token::Literal(literal) => literal,
            _ => return Err(ParserError::UnexpectedToken),
        };

        Ok(Proto::Literal(Literal::new(
            self.id_generator.generate(),
            &value,
        )))
    }

    fn next_or_err<T>(&mut self, expected_token_types: T) -> Result<Token, ParserError>
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

        Err(ParserError::UnexpectedToken)
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
            Ok(Token::Identifier("span".to_string())),
            Parser::new("span {}").next_or_err([TokenType::Identifier])
        );
    }

    #[test]
    fn parse_elements_with_attributes() {
        assert_eq!(
            Ok(Proto::Element(Element::new(
                1,
                "span",
                vec![Proto::Attribute(Attribute::new("style", "\"\"")),],
                vec![Proto::Element(Element::new(2, "div", vec![], vec![]))]
            ))),
            Parser::new(r#"span {@style="" div{}}"#).parse(),
        );

        assert_eq!(
            Ok(Proto::Element(Element::new(
                1,
                "span",
                vec![Proto::Attribute(Attribute::new("style", "\"\""))],
                vec![],
            ))),
            Parser::new(r#"span {@style=""}"#).parse(),
        );
    }

    #[test]
    fn parse_elements() {
        assert_eq!(
            Ok(Proto::Element(Element::new(
                1,
                "span",
                vec![],
                vec![Proto::Element(Element::new(2, "div", vec![], vec![]))]
            ))),
            Parser::new("span{div{}}").parse(),
        );

        assert_eq!(
            Ok(Proto::Element(Element::new(1, "div", vec![], vec![]))),
            Parser::new("div {}").parse()
        );
    }
}
