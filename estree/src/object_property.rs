use crate::{JsNode, js_node_type::JsNodeType, string_literal::StringLiteral};

#[derive(Debug, PartialEq)]
pub struct ObjectProperty {
    pub js_node_type: JsNodeType,
    pub key: Box<JsNode>,
    pub value: Box<JsNode>,
}

impl ObjectProperty {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            js_node_type: JsNodeType::ObjectProperty,
            key: Box::new(JsNode::StringLiteral(StringLiteral::new(key))),
            value: Box::new(JsNode::StringLiteral(StringLiteral::new(value))),
        }
    }
}
