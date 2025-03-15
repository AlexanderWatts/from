use estree::{
    JsNode, block_statement::BlockStatement, call_expression::CallExpression,
    function_declaration::FunctionDeclaration, identifier::Identifier,
    member_expression::MemberExpression, return_statement::ReturnStatement,
};
use proto::{Element, Proto, ProtoVisitor};

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &Proto) -> JsNode {
        let tree = root.accept(self);

        tree
    }
}

impl ProtoVisitor<JsNode> for Transpiler {
    ///
    /// ```js
    /// function createElement(element_type) {
    ///     return document.createElement(element_type);
    /// }
    /// ```
    ///
    fn visit_element(&self, element: &Element) -> JsNode {
        let mut block_statements = vec![];

        for element in element.block.iter() {
            block_statements.push(element.accept(self));
        }

        JsNode::FunctionDeclaration(FunctionDeclaration::new(
            Identifier::new("createElement"),
            BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(
                JsNode::CallExpression(CallExpression::new(
                    JsNode::MemberExpression(MemberExpression::new(
                        JsNode::Identifier(Identifier::new("document")),
                        Some(JsNode::Identifier(Identifier::new("createElement"))),
                    )),
                    vec![],
                )),
            ))]),
        ))
    }
}

#[cfg(test)]
mod transpiler {
    use super::*;

    #[test]
    fn transpile_element() {
        assert_eq!(
            JsNode::FunctionDeclaration(FunctionDeclaration::new(
                Identifier::new("createElement"),
                BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(
                    JsNode::CallExpression(CallExpression::new(
                        JsNode::MemberExpression(MemberExpression::new(
                            JsNode::Identifier(Identifier::new("document")),
                            Some(JsNode::Identifier(Identifier::new("createElement"))),
                        )),
                        vec![],
                    )),
                ))]),
            )),
            Transpiler.transpile(&Proto::Element(Element::new("div", vec![])))
        );
    }
}
