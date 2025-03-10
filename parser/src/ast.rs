#[derive(Debug, PartialEq)]
pub struct Element {
    pub block: Block,
}

impl Element {
    pub fn new(block: Block) -> Self {
        Self { block }
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            block: Block::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub elements: Vec<Element>,
}

impl Block {
    pub fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self { elements: vec![] }
    }
}

#[cfg(test)]
mod ast {
    use super::*;

    #[test]
    fn create_element_with_block() {
        assert_eq!(
            Element {
                block: Block { elements: vec![] },
            },
            Element::default()
        );
    }
}
