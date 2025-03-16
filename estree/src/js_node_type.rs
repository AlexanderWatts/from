#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    MemberExpression,
    StringLiteral,
    VariableDeclaration,
    FunctionDeclaration,
    ReturnStatement,
}
