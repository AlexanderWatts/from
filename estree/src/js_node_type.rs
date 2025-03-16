#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    CallExpression,
    MemberExpression,
    StringLiteral,
    VariableDeclaration,
    VariableDeclarator,
    FunctionDeclaration,
    ReturnStatement,
}
