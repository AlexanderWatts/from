pub trait NodeVisitor {
    fn visit_element<T>(&self, element: &Element) -> T;
    fn visit_block<T>(&self, block: &Block) -> T;
}

pub trait Node {
    fn accept<T>(&self, node_visitor: &impl NodeVisitor) -> T;
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub block: Block,
}

impl Node for Element {
    fn accept<T>(&self, node_visitor: &impl NodeVisitor) -> T {
        node_visitor.visit_element(self)
    }
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

impl Node for Block {
    fn accept<T>(&self, node_visitor: &impl NodeVisitor) -> T {
        node_visitor.visit_block(self)
    }
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
mod node {
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
