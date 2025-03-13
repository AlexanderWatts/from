#[derive(Debug, PartialEq)]
pub enum Proto {
    Element(Element),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub block: Vec<Proto>,
}

impl Default for Element {
    fn default() -> Self {
        Self { block: vec![] }
    }
}

impl Element {
    pub fn new(block: Vec<Proto>) -> Self {
        Self { block }
    }
}

impl Proto {}

#[cfg(test)]
mod proto {
    use super::*;

    #[test]
    fn setup() {
        assert_eq!(
            Proto::Element(Element {
                block: vec![Proto::Element(Element { block: vec![] })]
            }),
            Proto::Element(Element::new(vec![Proto::Element(Element::default())]))
        );
    }
}
