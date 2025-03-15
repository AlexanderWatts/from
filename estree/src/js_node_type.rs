#[derive(Debug, PartialEq)]
pub enum JsNodeType {
    Identifier,
    BlockStatement,
    FunctionDeclaration,
    ReturnStatement,
}
