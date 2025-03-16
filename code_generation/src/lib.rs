use estree::{
    JsNode, JsVisitor,
    block_statement::BlockStatement,
    call_expression::CallExpression,
    function_declaration::FunctionDeclaration,
    identifier::{self, Identifier},
    member_expression::MemberExpression,
    return_statement::ReturnStatement,
    string_literal::StringLiteral,
    variable_declaration::VariableDeclaration,
    variable_declarator::VariableDeclarator,
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

        format!("return {}", argument)
    }

    fn visit_call_expression(&self, call_expression: &CallExpression) -> String {
        let CallExpression {
            callee, arguments, ..
        } = call_expression;

        let callee = callee.accept(self);

        let arguments = arguments
            .into_iter()
            .map(|argument| argument.accept(self))
            .collect::<String>();

        format!("{callee}({arguments})")
    }

    fn visit_member_expression(&self, member_expression: &MemberExpression) -> String {
        let MemberExpression {
            object, property, ..
        } = member_expression;

        let object = object.accept(self);

        let property = match property {
            Some(p) => format!(".{}", p.accept(self)),
            None => "".to_string(),
        };

        format!("{object}{property}")
    }

    fn visit_string_literal(&self, string_literal: &StringLiteral) -> String {
        format!(r#""{}""#, string_literal.value)
    }

    fn visit_variable_declaration(&self, variable_declaration: &VariableDeclaration) -> String {
        let VariableDeclaration {
            kind, declarations, ..
        } = variable_declaration;

        let declarations = declarations
            .into_iter()
            .map(|declaration| declaration.accept(self))
            .collect::<Vec<String>>()
            .join(", ");

        format!("{kind} {declarations}")
    }

    fn visit_variable_declarator(&self, variable_declarator: &VariableDeclarator) -> String {
        let VariableDeclarator {
            identifier,
            initialiser,
            ..
        } = variable_declarator;

        let identifier = identifier.accept(self);
        let initialiser = initialiser.accept(self);

        format!("{identifier} = {initialiser}")
    }
}

#[cfg(test)]
mod tests {
    use estree::variable_declaration::VariableDeclarationKind;

    use super::*;

    #[test]
    fn generate_variable() {
        assert_eq!(
            r#"let x = "hello""#,
            CodeGenerator.generate(&JsNode::VariableDeclaration(VariableDeclaration::new(
                VariableDeclarationKind::Let,
                vec![JsNode::VariableDeclarator(VariableDeclarator::new(
                    Identifier::new("x"),
                    JsNode::StringLiteral(StringLiteral::new("hello"))
                )),]
            )))
        )
    }

    #[test]
    fn generate_call_expression_with_members() {
        assert_eq!(
            r#"document.main.createElement("div")"#,
            CodeGenerator.generate(&JsNode::CallExpression(CallExpression::new(
                JsNode::MemberExpression(MemberExpression::new(
                    JsNode::Identifier(Identifier::new("document")),
                    Some(JsNode::MemberExpression(MemberExpression::new(
                        JsNode::Identifier(Identifier::new("main")),
                        Some(JsNode::MemberExpression(MemberExpression::new(
                            JsNode::Identifier(Identifier::new("createElement")),
                            None
                        ))),
                    ))),
                )),
                vec![JsNode::StringLiteral(StringLiteral::new("div"))],
            )))
        );
    }

    #[test]
    fn generate_call_expression() {
        assert_eq!(
            r#"document.createElement("div")"#,
            CodeGenerator.generate(&JsNode::CallExpression(CallExpression::new(
                JsNode::MemberExpression(MemberExpression::new(
                    JsNode::Identifier(Identifier::new("document")),
                    Some(JsNode::Identifier(Identifier::new("createElement"))),
                )),
                vec![JsNode::StringLiteral(StringLiteral::new("div"))],
            )))
        );
    }

    #[test]
    fn generate_return_statement() {
        assert_eq!(
            "return x",
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
