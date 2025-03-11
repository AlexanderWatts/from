use node::{Node, NodeVisitor};

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, root: &impl Node) -> String {
        root.accept(self)
    }
}

impl NodeVisitor<String> for Transpiler {
    fn visit_element(&self, element: &node::Element) -> String {
        format!("")
    }

    fn visit_block(&self, block: &node::Block) -> String {
        format!("")
    }
}

#[cfg(test)]
mod transpiler {
    use node::Element;

    use super::*;

    #[test]
    fn transpile() {
        assert_eq!("", Transpiler.transpile(&Element::default()));
    }
}
