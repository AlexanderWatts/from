#[derive(Debug, PartialEq)]
pub enum Proto {
    Element(Element),
    Literal(String),
}

pub trait ProtoVisitor<T> {
    fn visit_element(&self, element: &Element) -> T;
    fn visit_literal(&self, literal: &String) -> T;
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
            Self::Literal(string) => visitor.visit_literal(string),
        }
    }
}
