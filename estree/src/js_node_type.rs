#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    MemberExpression,
    StringLiteral,
    NullLiteral,
    VariableDeclaration,
    VariableDeclarator,
    FunctionDeclaration,
    ReturnStatement,
}
