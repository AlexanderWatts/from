use lexer::Lexer;

/// 
/// Grammar
/// 
/// program := element
/// element := ('div' | 'span') element_block
/// element_block := '{' element* '}'
///
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

