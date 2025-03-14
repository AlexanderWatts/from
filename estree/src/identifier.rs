use crate::js_node_type::JsNodeType;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub js_node_type: JsNodeType,
    pub name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Self {
            js_node_type: JsNodeType::Identifier,
            name: name.to_string(),
        }
    }
}
