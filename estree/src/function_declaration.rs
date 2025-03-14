use crate::{
    JsNode, block_statement::BlockStatement, identifier::Identifier, js_node_type::JsNodeType,
};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub js_node_type: JsNodeType,
    pub identifier: Box<JsNode>,
    pub body: Box<JsNode>,
}

impl FunctionDeclaration {
    pub fn new(identifier: Identifier, body: BlockStatement) -> Self {
        Self {
            js_node_type: JsNodeType::FunctionDeclaration,
            identifier: Box::new(JsNode::Identifier(identifier)),
            body: Box::new(JsNode::BlockStatement(body)),
        }
    }
}
