pub mod block_statement;
pub mod call_expression;
pub mod function_declaration;
pub mod identifier;
pub mod js_node_type;
pub mod member_expression;
pub mod null_literal;
pub mod object_expression;
pub mod object_property;
pub mod return_statement;
pub mod string_literal;
pub mod variable_declaration;
pub mod variable_declarator;

use block_statement::BlockStatement;
use call_expression::CallExpression;
use function_declaration::FunctionDeclaration;
use identifier::Identifier;
use member_expression::MemberExpression;
use null_literal::NullLiteral;
use object_expression::ObjectExpression;
use object_property::ObjectProperty;
use return_statement::ReturnStatement;
use string_literal::StringLiteral;
use variable_declaration::VariableDeclaration;
use variable_declarator::VariableDeclarator;

pub trait JsVisitor<T> {
    fn visit_block_statement(&self, block_statement: &BlockStatement) -> T;
    fn visit_function_declaration(&self, function_declaration: &FunctionDeclaration) -> T;
    fn visit_identifier(&self, identifier: &Identifier) -> T;
    fn visit_return_statement(&self, return_statement: &ReturnStatement) -> T;
    fn visit_call_expression(&self, call_expression: &CallExpression) -> T;
    fn visit_member_expression(&self, member_expression: &MemberExpression) -> T;
    fn visit_string_literal(&self, string_literal: &StringLiteral) -> T;
    fn visit_variable_declaration(&self, variable_declaration: &VariableDeclaration) -> T;
    fn visit_variable_declarator(&self, variable_declarator: &VariableDeclarator) -> T;
    fn visit_null_literal(&self, null_literal: &NullLiteral) -> T;
    fn visit_object_expression(&self, object_expression: &ObjectExpression) -> T;
    fn visit_object_property(&self, object_property: &ObjectProperty) -> T;
}

#[derive(Debug, PartialEq)]
pub enum JsNode {
    BlockStatement(BlockStatement),
    FunctionDeclaration(FunctionDeclaration),
    Identifier(Identifier),
    ReturnStatement(ReturnStatement),
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
    StringLiteral(StringLiteral),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
    NullLiteral(NullLiteral),
    ObjectExpression(ObjectExpression),
    ObjectProperty(ObjectProperty),
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
            Self::MemberExpression(member_expression) => {
                visitor.visit_member_expression(member_expression)
            }
            Self::StringLiteral(string_literal) => visitor.visit_string_literal(string_literal),
            Self::VariableDeclaration(variable_declaration) => {
                visitor.visit_variable_declaration(variable_declaration)
            }
            Self::VariableDeclarator(variable_declarator) => {
                visitor.visit_variable_declarator(variable_declarator)
            }
            Self::NullLiteral(null_literal) => visitor.visit_null_literal(null_literal),
            Self::ObjectExpression(object_expression) => {
                visitor.visit_object_expression(object_expression)
            }
            Self::ObjectProperty(object_property) => visitor.visit_object_property(object_property),
        }
    }
}
