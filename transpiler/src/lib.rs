use estree::{BlockStatement, FunctionExpression, JsNode};
use proto::{Element, Proto, ProtoVisitor};

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &Proto) -> JsNode {
        let tree = root.accept(self);

        tree
    }
}

impl ProtoVisitor<JsNode> for Transpiler {
    fn visit_element(&self, element: &Element) -> JsNode {
        let mut block_statements = vec![];

        for element in element.block.iter() {
            block_statements.push(element.accept(self));
        }

        JsNode::FunctionExpression(FunctionExpression::new(JsNode::BlockStatement(
            BlockStatement::new(block_statements),
        )))
    }
}

#[cfg(test)]
mod transpiler {
    use super::*;

    #[test]
    fn transpile() {
        assert_eq!(
            JsNode::FunctionExpression(FunctionExpression::new(JsNode::BlockStatement(
                BlockStatement::new(vec![JsNode::FunctionExpression(FunctionExpression::new(
                    JsNode::BlockStatement(BlockStatement::new(vec![]),)
                )),]),
            ))),
            Transpiler.transpile(&Proto::Element(Element {
                block: vec![Proto::Element(Element { block: vec![] })]
            }))
        );
    }
}
