#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    body: Vec<JsNode>,
}

impl BlockStatement {
    pub fn new(body: Vec<JsNode>) -> Self {
        Self { body }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    body: Box<JsNode>,
}

impl FunctionExpression {
    pub fn new(body: JsNode) -> Self {
        Self {
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionExpression(FunctionExpression),
}
