use ast::{Block, Element};
use lexer::Lexer;
mod ast;

///
/// Grammar
///
/// program := element
/// element := ('div' | 'span') element_block
/// element_block := '{' element* '}'
///
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&self) -> Result<Element, ()> {
        self.element()
    }

    fn element(&self) -> Result<Element, ()> {
        Ok(Element::default())
    }

    fn element_block(&self) -> Result<Block, ()> {
        Ok(Block::default())
    }
}

#[cfg(test)]
mod parser {
    use super::*;

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
