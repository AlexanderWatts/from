use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub js_node_type: JsNodeType,
    pub callee: Box<JsNode>,
    pub arguments: Vec<JsNode>,
}

impl CallExpression {
    pub fn new(callee: JsNode, arguments: Vec<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::CallExpression,
            callee: Box::new(callee),
            arguments,
        }
    }
}
