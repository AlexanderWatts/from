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

pub trait JsVisitor {
    fn visit_block_statement(&self, block_statement: &BlockStatement);
    fn visit_function_expression(&self, function_expression: &FunctionExpression);
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionExpression(FunctionExpression),
}

impl JsNode {
    pub fn accept(&self, visitor: &impl JsVisitor) {
        match self {
            Self::BlockStatement(block_statement) => visitor.visit_block_statement(block_statement),
            Self::FunctionExpression(function_expression) => {
                visitor.visit_function_expression(function_expression)
            }
        }
    }
}
