use estree::{
    JsNode, block_statement::BlockStatement, function_declaration::FunctionDeclaration,
    identifier::Identifier, return_statement::ReturnStatement,
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
    ///     let element = document.createElement(element_type);
    ///     return element;
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
                JsNode::Identifier(Identifier::new("element")),
            ))]),
        ))
    }
}

#[cfg(test)]
mod transpiler {
    use super::*;

    #[test]
    fn transpile() {
        assert_eq!(
            JsNode::FunctionDeclaration(FunctionDeclaration::new(
                Identifier::new("createElement"),
                BlockStatement::new(vec![JsNode::ReturnStatement(ReturnStatement::new(
                    JsNode::Identifier(Identifier::new("element")),
                ))])
            )),
            Transpiler.transpile(&Proto::Element(Element {
                block: vec![Proto::Element(Element { block: vec![] })]
            }))
        );
    }
}
