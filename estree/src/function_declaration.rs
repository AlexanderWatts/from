use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    js_node_type: JsNodeType,
    body: Box<JsNode>,
}

impl FunctionDeclaration {
    pub fn new(body: JsNode) -> Self {
        Self {
            js_node_type: JsNodeType::FunctionDeclaration,
            body: Box::new(body),
        }
    }
}
