use crate::js_node_type::JsNodeType;

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub js_node_type: JsNodeType,
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            js_node_type: JsNodeType::StringLiteral,
            value: String::from(value),
        }
    }
}
