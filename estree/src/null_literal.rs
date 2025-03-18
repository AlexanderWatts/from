use crate::js_node_type::JsNodeType;

#[derive(Debug, PartialEq)]
pub struct NullLiteral {
    pub js_node_type: JsNodeType,
    pub value: String,
}

impl NullLiteral {
    pub fn new() -> Self {
        Self {
            js_node_type: JsNodeType::NullLiteral,
            value: String::from("null"),
        }
    }
}
