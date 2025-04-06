use estree::{
    JsNode,
    block_statement::BlockStatement,
    call_expression::CallExpression,
    function_declaration::FunctionDeclaration,
    identifier::Identifier,
    string_literal::StringLiteral,
    variable_declaration::{VariableDeclaration, VariableDeclarationKind},
    variable_declarator::VariableDeclarator,
};
use proto::{Element, Literal, Proto};
use std::collections::VecDeque;

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &Proto) -> JsNode {
        let block = self.block(root);

        self.function_wrapper(block)
    }

    /// ```js
    /// function dom() {
    ///     // Block code
    /// }
    fn function_wrapper(&self, block: Vec<JsNode>) -> JsNode {
        JsNode::FunctionDeclaration(FunctionDeclaration::new(
            Identifier::new("dom"),
            BlockStatement::new(block),
        ))
    }

    fn block(&self, proto: &Proto) -> Vec<JsNode> {
        let mut queue: VecDeque<(&Proto, Option<String>)> = VecDeque::new();
        let mut instantiate: Vec<JsNode> = vec![];
        let mut block: Vec<JsNode> = vec![];
        let mut dom: Vec<JsNode> = vec![];

        queue.push_back((proto, None));

        while let Some((proto, parent_id)) = queue.pop_front() {
            match proto {
                Proto::Element(element) => {
                    let id = format!("{}{}", element.element_type, element.element_id);

                    instantiate.push(JsNode::VariableDeclaration(VariableDeclaration::new(
                        VariableDeclarationKind::Let,
                        vec![JsNode::VariableDeclarator(VariableDeclarator::new(
                            Identifier::new(&id),
                            Some(self.visit_element(element)),
                        ))],
                    )));

                    element
                        .attributes
                        .iter()
                        .for_each(|attr| queue.push_back((attr, Some(id.to_owned()))));

                    element
                        .children
                        .iter()
                        .for_each(|attr| queue.push_back((attr, Some(id.to_owned()))));

                    dom.push(JsNode::CallExpression(CallExpression::new(
                        JsNode::Identifier(Identifier::new("append")),
                        vec![
                            JsNode::StringLiteral(StringLiteral::new(
                                &parent_id.unwrap_or("target".to_string()),
                            )),
                            JsNode::StringLiteral(StringLiteral::new(&id)),
                        ],
                    )));
                }
                Proto::Literal(literal) => {
                    let id = format!("t{}", literal.literal_id);

                    instantiate.push(JsNode::VariableDeclaration(VariableDeclaration::new(
                        VariableDeclarationKind::Let,
                        vec![JsNode::VariableDeclarator(VariableDeclarator::new(
                            Identifier::new(&id),
                            Some(self.visit_literal(literal)),
                        ))],
                    )));

                    dom.push(JsNode::CallExpression(CallExpression::new(
                        JsNode::Identifier(Identifier::new("append")),
                        vec![
                            JsNode::StringLiteral(StringLiteral::new(
                                &parent_id.unwrap_or("target".to_string()),
                            )),
                            JsNode::StringLiteral(StringLiteral::new(&id)),
                        ],
                    )));
                }
                Proto::Attribute(attribute) => {
                    block.push(self.visit_attribute(attribute, &parent_id.unwrap()))
                }
            }
        }

        instantiate.append(&mut block);
        instantiate.append(&mut dom);

        instantiate
    }

    /// The `element` function declaration is provided by the runtime library
    ///
    /// ```js
    /// element("div")
    /// ```
    fn visit_element(&self, element: &Element) -> JsNode {
        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("element")),
            vec![JsNode::StringLiteral(StringLiteral::new(
                format!("\"{}\"", &element.element_type).as_str(),
            ))],
        ))
    }

    /// The `literal` function declaration is provided by the runtime library
    ///
    /// ```js
    /// literal("Hello, World!")
    /// ```
    fn visit_literal(&self, literal: &Literal) -> JsNode {
        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("literal")),
            vec![JsNode::StringLiteral(StringLiteral::new(&literal.literal))],
        ))
    }

    /// The `attribute` function declaration is provided by the runtime library
    ///
    /// ```js
    /// attribute(parent_id, "class", "flex")
    /// ```
    fn visit_attribute(&self, attribute: &proto::Attribute, parent_id: &str) -> JsNode {
        JsNode::CallExpression(CallExpression::new(
            JsNode::Identifier(Identifier::new("attribute")),
            vec![
                JsNode::StringLiteral(StringLiteral::new(parent_id)),
                JsNode::StringLiteral(StringLiteral::new(
                    format!("\"{}\"", &attribute.name).as_str(),
                )),
                JsNode::StringLiteral(StringLiteral::new(&attribute.value)),
            ],
        ))
    }
}

#[cfg(test)]
mod transpiler {
    use estree::{
        JsNode, block_statement::BlockStatement, call_expression::CallExpression,
        function_declaration::FunctionDeclaration, identifier::Identifier,
        object_expression::ObjectExpression, object_property::ObjectProperty,
        return_statement::ReturnStatement, string_literal::StringLiteral,
    };
    use proto::{Attribute, Literal};

    use super::*;

    #[test]
    fn z() {
        let _r = Transpiler.block(&Proto::Element(Element::new(
            1,
            "div",
            vec![Proto::Attribute(Attribute::new("style", "\"color: red;\""))],
            vec![Proto::Element(Element::new(
                2,
                "span",
                vec![Proto::Attribute(Attribute::new("class", "\"flex\""))],
                vec![Proto::Literal(Literal::new(1, "Hello World!"))],
            ))],
        )));
    }

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
                1,
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
                1,
                "div",
                vec![],
                vec![Proto::Element(Element::new(2, "span", vec![], vec![]))]
            )))
        );
    }
}
