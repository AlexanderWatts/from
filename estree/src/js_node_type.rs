#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    MemberExpression,
    ObjectExpression,
    StringLiteral,
    NullLiteral,
    VariableDeclaration,
    VariableDeclarator,
    FunctionDeclaration,
    ReturnStatement,
}
