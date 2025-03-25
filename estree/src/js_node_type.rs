#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    MemberExpression,
    ObjectProperty,
    ObjectExpression,
    StringLiteral,
    NullLiteral,
    VariableDeclaration,
    VariableDeclarator,
    FunctionDeclaration,
    ReturnStatement,
}
