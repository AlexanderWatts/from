pub mod block_statement;
pub mod call_expression;
pub mod function_declaration;
pub mod identifier;
pub mod js_node_type;
pub mod return_statement;

use block_statement::BlockStatement;
use call_expression::CallExpression;
use function_declaration::FunctionDeclaration;
use identifier::Identifier;
use return_statement::ReturnStatement;

pub trait JsVisitor<T> {
    fn visit_block_statement(&self, block_statement: &BlockStatement) -> T;
    fn visit_function_declaration(&self, function_declaration: &FunctionDeclaration) -> T;
    fn visit_identifier(&self, identifier: &Identifier) -> T;
    fn visit_return_statement(&self, return_statement: &ReturnStatement) -> T;
    fn visit_call_expression(&self, call_expression: &CallExpression) -> T;
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionDeclaration(FunctionDeclaration),
    Identifier(Identifier),
    ReturnStatement(ReturnStatement),
    CallExpression(CallExpression),
}

impl JsNode {
    pub fn accept<T>(&self, visitor: &impl JsVisitor<T>) -> T {
        match self {
            Self::BlockStatement(block_statement) => visitor.visit_block_statement(block_statement),
            Self::FunctionDeclaration(function_declaration) => {
                visitor.visit_function_declaration(function_declaration)
            }
            Self::Identifier(identifier) => visitor.visit_identifier(identifier),
            Self::ReturnStatement(return_statement) => {
                visitor.visit_return_statement(return_statement)
            }
            Self::CallExpression(call_expression) => visitor.visit_call_expression(call_expression),
        }
    }
}
