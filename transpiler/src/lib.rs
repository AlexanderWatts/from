use estree::{
    JsNode, block_statement::BlockStatement, call_expression::CallExpression,
    function_declaration::FunctionDeclaration, identifier::Identifier,
    return_statement::ReturnStatement, string_literal::StringLiteral,
};
use proto::{Element, Proto, ProtoVisitor};

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &Proto) -> JsNode {
        let tree = root.accept(self);

        self.function_wrapper(tree)
    }

    /// Wrap the dom tree in a function declararion
    ///
    /// ```js
    /// function dom() {
    ///     return element("div", null,
    ///         element("span", {})
    ///     );
    /// }
    fn function_wrapper(&self, root: JsNode) -> JsNode {
        JsNode::FunctionDeclaration(FunctionDeclaration::new(
            Identifier::new("dom"),
            BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(root))]),
        ))
    }
}

impl ProtoVisitor<JsNode> for Transpiler {
    /// The `element` function declaration is provided by the runtime library
    ///
    /// ```js
    /// element("div", null,
    ///     element("span", {})
    /// );
    /// ```
    fn visit_element(&self, element: &Element) -> JsNode {
        let mut arguments = vec![JsNode::StringLiteral(StringLiteral::new(
            &element.element_type,
        ))];

        arguments.extend(
            element
                .block
                .iter()
                .map(|ele| ele.accept(self))
                .collect::<Vec<JsNode>>(),
        );

        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("element")),
            arguments,
        ))
    }
}

#[cfg(test)]
mod transpiler {
    use super::*;

    #[test]
    fn transpile_elements() {
        assert_eq!(
            JsNode::FunctionDeclaration(FunctionDeclaration::new(
                Identifier::new("dom"),
                BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(
                    JsNode::CallExpression(CallExpression::new(
                        JsNode::Identifier(Identifier::new("element")),
                        vec![
                            JsNode::StringLiteral(StringLiteral::new("div")),
                            JsNode::CallExpression(CallExpression::new(
                                JsNode::Identifier(Identifier::new("element")),
                                vec![JsNode::StringLiteral(StringLiteral::new("span"))],
                            )),
                        ],
                    ))
                ))]),
            )),
            Transpiler.transpile(&Proto::Element(Element::new(
                "div",
                vec![Proto::Element(Element::new("span", vec![]))]
            )))
        );
    }
}
