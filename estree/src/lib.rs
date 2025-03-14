pub mod block_statement;
pub mod function_declaration;
pub mod js_node_type;

use block_statement::BlockStatement;
use function_declaration::FunctionDeclaration;

pub trait JsVisitor<T> {
    fn visit_block_statement(&self, block_statement: &BlockStatement) -> T;
    fn visit_function_declaration(&self, function_declaration: &FunctionDeclaration) -> T;
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionDeclaration(FunctionDeclaration),
}

impl JsNode {
    pub fn accept<T>(&self, visitor: &impl JsVisitor<T>) -> T {
        match self {
            Self::BlockStatement(block_statement) => visitor.visit_block_statement(block_statement),
            Self::FunctionDeclaration(function_declaration) => {
                visitor.visit_function_declaration(function_declaration)
            }
        }
    }
}
