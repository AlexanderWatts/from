#[derive(Debug, PartialEq)]
pub enum Proto {
    Element(Element),
    Literal(String),
    Attribute(Attribute),
}

pub trait ProtoVisitor<T> {
    fn visit_element(&self, element: &Element) -> T;
    fn visit_literal(&self, literal: &String) -> T;
    fn visit_attribute(&self, attribute: &Attribute) -> T;
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

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    value: Box<Proto>,
}

impl Attribute {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: Box::new(Proto::Literal(value.to_string())),
        }
    }
}

impl Proto {
    pub fn accept<T>(&self, visitor: &impl ProtoVisitor<T>) -> T {
        match self {
            Self::Element(element) => visitor.visit_element(element),
            Self::Literal(string) => visitor.visit_literal(string),
            Self::Attribute(attribute) => visitor.visit_attribute(attribute),
        }
    }
}
