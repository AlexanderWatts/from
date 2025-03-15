#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    FunctionDeclaration,
    ReturnStatement,
}
