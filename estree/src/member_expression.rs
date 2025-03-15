use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct MemberExpression {
    pub js_node_type: JsNodeType,
    pub object: Box<JsNode>,
    pub property: Option<Box<JsNode>>,
}

impl MemberExpression {
    pub fn new(object: JsNode, property: Option<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::MemberExpression,
            object: Box::new(object),
            property: property.map(|js_node| Box::new(js_node)),
        }
    }
}
