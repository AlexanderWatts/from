use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub js_node_type: JsNodeType,
    pub body: Vec<JsNode>,
}

impl BlockStatement {
    pub fn new(body: Vec<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::BlockStatement,
            body,
        }
    }
}
