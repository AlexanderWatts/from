#[derive(Debug, PartialEq)]
pub enum Proto {
    Element(Element),
    Literal(Literal),
    Attribute(Attribute),
}

pub trait ProtoVisitor<T> {
    fn visit_element(&self, element: &Element) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_attribute(&self, attribute: &Attribute) -> T;
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub literal_id: usize,
    pub literal: String,
}

impl Literal {
    pub fn new(literal_id: usize, literal: &str) -> Self {
        Self {
            literal_id,
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub element_id: usize,
    pub element_type: String,
    pub attributes: Vec<Proto>,
    pub children: Vec<Proto>,
}

impl Element {
    pub fn new(
        element_id: usize,
        element_type: &str,
        attributes: Vec<Proto>,
        children: Vec<Proto>,
    ) -> Self {
        Self {
            element_id,
            element_type: String::from(element_type),
            attributes,
            children,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl Proto {
    pub fn accept<T>(&self, visitor: &impl ProtoVisitor<T>) -> T {
        match self {
            Self::Element(element) => visitor.visit_element(element),
            Self::Literal(literal) => visitor.visit_literal(literal),
            Self::Attribute(attribute) => visitor.visit_attribute(attribute),
        }
    }
}
