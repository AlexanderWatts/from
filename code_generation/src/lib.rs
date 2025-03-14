use estree::{JsNode, JsVisitor};

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
    fn visit_block_statement(&self, block_statement: &estree::BlockStatement) -> String {
        format!("")
    }

    fn visit_function_expression(
        &self,
        function_expression: &estree::FunctionExpression,
    ) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _code_generator = CodeGenerator;
    }
}
