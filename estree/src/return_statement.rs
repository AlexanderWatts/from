use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub js_node_type: JsNodeType,
    pub argument: Box<JsNode>,
}

impl ReturnStatement {
    pub fn new(argument: JsNode) -> Self {
        Self {
            js_node_type: JsNodeType::ReturnStatement,
            argument: Box::new(argument),
        }
    }
}
