use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct ObjectExpression {
    pub js_node_type: JsNodeType,
    pub properties: Vec<JsNode>,
}

impl ObjectExpression {
    pub fn new(properties: Vec<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::ObjectExpression,
            properties,
        }
    }
}
