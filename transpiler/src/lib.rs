use estree::{
    JsNode, block_statement::BlockStatement, call_expression::CallExpression,
    function_declaration::FunctionDeclaration, identifier::Identifier,
    object_expression::ObjectExpression, object_property::ObjectProperty,
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
    ///     return element(
    ///         "div",
    ///         {},
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
    /// element(
    ///     "div",
    ///     { class: "" },
    ///     element("span", {})
    /// );
    /// ```
    fn visit_element(&self, element: &Element) -> JsNode {
        let mut arguments = vec![JsNode::StringLiteral(StringLiteral::new(
            &element.element_type,
        ))];

        let attributes = ObjectExpression::new(
            element
                .attributes
                .iter()
                .map(|ele| ele.accept(self))
                .collect::<Vec<JsNode>>(),
        );
        let elements = element
            .children
            .iter()
            .map(|ele| ele.accept(self))
            .collect::<Vec<JsNode>>();

        arguments.push(JsNode::ObjectExpression(attributes));
        arguments.extend(elements);

        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("element")),
            arguments,
        ))
    }

    /// The `literal` function declaration is provided by the runtime library
    ///
    /// ```js
    /// literal("Hello, World!")
    /// ```
    fn visit_literal(&self, literal: &String) -> JsNode {
        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("literal")),
            vec![JsNode::StringLiteral(StringLiteral::new(literal))],
        ))
    }

    /// ```js
    /// {
    ///     class: "flex"
    ///     style: "padding"
    /// }
    /// ```
    fn visit_attribute(&self, attribute: &proto::Attribute) -> JsNode {
        JsNode::ObjectProperty(ObjectProperty::new(&attribute.name, &attribute.value))
    }
}

#[cfg(test)]
mod transpiler {
    use proto::Attribute;

    use super::*;

    #[test]
    fn transpile_elements_with_attributes() {
        assert_eq!(
            JsNode::FunctionDeclaration(FunctionDeclaration::new(
                Identifier::new("dom"),
                BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(
                    JsNode::CallExpression(CallExpression::new(
                        JsNode::Identifier(Identifier::new("element")),
                        vec![
                            JsNode::StringLiteral(StringLiteral::new("div")),
                            JsNode::ObjectExpression(ObjectExpression::new(vec![
                                JsNode::ObjectProperty(ObjectProperty::new(
                                    "style",
                                    "\"color: red;\""
                                ))
                            ])),
                        ],
                    ))
                ))]),
            )),
            Transpiler.transpile(&Proto::Element(Element::new(
                "div",
                vec![Proto::Attribute(Attribute::new("style", "\"color: red;\"")),],
                vec![],
            )))
        );
    }

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
                            JsNode::ObjectExpression(ObjectExpression::new(vec![])),
                            JsNode::CallExpression(CallExpression::new(
                                JsNode::Identifier(Identifier::new("element")),
                                vec![
                                    JsNode::StringLiteral(StringLiteral::new("span")),
                                    JsNode::ObjectExpression(ObjectExpression::new(vec![])),
                                ],
                            )),
                        ],
                    ))
                ))]),
            )),
            Transpiler.transpile(&Proto::Element(Element::new(
                "div",
                vec![],
                vec![Proto::Element(Element::new("span", vec![], vec![]))]
            )))
        );
    }
}
