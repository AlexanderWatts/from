mod js_node;
use js_node::{BlockStatement, FunctionExpression, JsNode};
use node::{Node, NodeVisitor};

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &impl Node) -> JsNode {
        root.accept(self)
    }
}

impl NodeVisitor<JsNode> for Transpiler {
    fn visit_element(&self, element: &node::Element) -> JsNode {
        let body = element.block.accept(self);

        JsNode::FunctionExpression(FunctionExpression::new(body))
    }

    fn visit_block(&self, block: &node::Block) -> JsNode {
        let mut elements = vec![];

        for element in block.elements.iter() {
            elements.push(element.accept(self));
        }

        JsNode::BlockStatement(BlockStatement::new(vec![]))
    }
}

#[cfg(test)]
mod transpiler {
    use node::Element;

    use super::*;

    #[test]
    fn transpile() {
        assert_eq!(
            JsNode::FunctionExpression(FunctionExpression::new(JsNode::BlockStatement(
                BlockStatement::new(vec![])
            ))),
            Transpiler.transpile(&Element::default())
        );
    }
}
