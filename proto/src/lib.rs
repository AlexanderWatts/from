#[derive(Debug, PartialEq)]
pub enum Proto {
    Element(Element),
}

pub trait ProtoVisitor<T> {
    fn visit_element(&self, element: &Element) -> T;
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub element_type: String,
    pub block: Vec<Proto>,
}

impl Element {
    pub fn new(element_type: &str, block: Vec<Proto>) -> Self {
        Self {
            element_type: String::from(element_type),
            block,
        }
    }
}

impl Proto {
    pub fn accept<T>(&self, visitor: &impl ProtoVisitor<T>) -> T {
        match self {
            Self::Element(element) => visitor.visit_element(element),
        }
    }
}

#[cfg(test)]
mod proto {
    use super::*;

    #[test]
    fn setup() {
        assert_eq!(
            Proto::Element(Element {
                element_type: "div".to_string(),
                block: vec![Proto::Element(Element {
                    element_type: "span".to_string(),
                    block: vec![]
                })]
            }),
            Proto::Element(Element::new(
                "div",
                vec![Proto::Element(Element::new("span", vec![]))]
            ))
        );
    }
}
