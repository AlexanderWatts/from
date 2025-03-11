#[derive(Debug, PartialEq)]
pub enum NodeType {
    BlockStatement,
    FunctionExpression,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    node_type: NodeType,
    body: Vec<JsNode>,
}

impl BlockStatement {
    pub fn new(body: Vec<JsNode>) -> Self {
        Self {
            node_type: NodeType::BlockStatement,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    node_type: NodeType,
    body: Box<JsNode>,
}

impl FunctionExpression {
    pub fn new(body: JsNode) -> Self {
        Self {
            node_type: NodeType::FunctionExpression,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionExpression(FunctionExpression),
}
