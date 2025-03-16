use std::fmt::Display;

use crate::{JsNode, js_node_type::JsNodeType};

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

impl Display for VariableDeclarationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "var"),
            Self::Let => write!(f, "let"),
            Self::Const => write!(f, "const"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub js_node_type: JsNodeType,
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<JsNode>,
}

impl VariableDeclaration {
    pub fn new(kind: VariableDeclarationKind, declarations: Vec<JsNode>) -> Self {
        Self {
            js_node_type: JsNodeType::VariableDeclaration,
            kind,
            declarations,
        }
    }
}
