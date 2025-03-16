use crate::{JsNode, identifier::Identifier, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator {
    pub js_node_type: JsNodeType,
    pub identifier: Box<JsNode>,
    pub initialiser: Box<JsNode>,
}

impl VariableDeclarator {
    pub fn new(identifier: Identifier, initialiser: JsNode) -> Self {
        Self {
            js_node_type: JsNodeType::VariableDeclarator,
            identifier: Box::new(JsNode::Identifier(identifier)),
            initialiser: Box::new(initialiser),
        }
    }
}
