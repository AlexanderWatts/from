use crate::{JsNode, identifier::Identifier, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator {
    pub js_node_type: JsNodeType,
    pub identifier: Box<JsNode>,
    pub initialiser: Option<Box<JsNode>>,
}

impl VariableDeclarator {
    pub fn new(identifier: Identifier, initialiser: Option<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::VariableDeclarator,
            identifier: Box::new(JsNode::Identifier(identifier)),
            initialiser: initialiser.map(Box::new),
        }
    }
}
