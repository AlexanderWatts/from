use estree::{
    JsNode, JsVisitor, block_statement::BlockStatement, function_declaration::FunctionDeclaration,
    identifier::Identifier, return_statement::ReturnStatement,
};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, root: &JsNode) -> String {
        root.accept(self)
    }
}

impl JsVisitor<String> for CodeGenerator {
    fn visit_block_statement(&self, block_statement: &BlockStatement) -> String {
        let BlockStatement { body, .. } = block_statement;

        body.into_iter()
            .map(|item| item.accept(self))
            .collect::<String>()
    }

    fn visit_function_declaration(&self, function_declaration: &FunctionDeclaration) -> String {
        let FunctionDeclaration {
            identifier, body, ..
        } = function_declaration;

        let identifier = identifier.accept(self);

        let body = body.accept(self);

        format!("function {identifier}() {{{body}}}")
    }

    fn visit_identifier(&self, identifier: &Identifier) -> String {
        format!("{}", identifier.name)
    }

    fn visit_return_statement(&self, return_statement: &ReturnStatement) -> String {
        let ReturnStatement { argument, .. } = return_statement;

        let argument = argument.accept(self);

        format!("return {};", argument)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_return_statement() {
        assert_eq!(
            "return x;",
            CodeGenerator.generate(&JsNode::ReturnStatement(ReturnStatement::new(
                JsNode::Identifier(Identifier::new("x"))
            )))
        );
    }

    #[test]
    fn generate_function_declaration() {
        assert_eq!(
            "function xyz() {}",
            CodeGenerator.generate(&JsNode::FunctionDeclaration(FunctionDeclaration::new(
                Identifier::new("xyz"),
                BlockStatement::new(vec![]),
            )))
        );
    }
}
