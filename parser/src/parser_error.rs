use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken,
    UnknownAttribute,
    UnknownElement,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken => write!(f, "Unexpected token"),
            ParserError::UnknownAttribute => write!(f, "Unknown attribute"),
            ParserError::UnknownElement => write!(f, "Unknown element"),
        }
    }
}
