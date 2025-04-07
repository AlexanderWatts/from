use crate::parser_error::ParserError;
use std::fmt::Display;

#[derive(Debug)]
pub enum HtmlTag {
    P,
    Form,
    Input,
    Button,
    Div,
    Span,
}

impl TryFrom<&str> for HtmlTag {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "div" => Ok(HtmlTag::Div),
            "span" => Ok(HtmlTag::Span),
            "form" => Ok(HtmlTag::Form),
            "input" => Ok(HtmlTag::Input),
            "button" => Ok(HtmlTag::Button),
            "p" => Ok(HtmlTag::P),
            _ => return Err(ParserError::UnknownElement),
        }
    }
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let html_tag = match self {
            HtmlTag::P => String::from("p"),
            HtmlTag::Form => String::from("form"),
            HtmlTag::Input => String::from("input"),
            HtmlTag::Button => String::from("button"),
            HtmlTag::Div => String::from("div"),
            HtmlTag::Span => String::from("span"),
        };
        write!(f, "{}", html_tag)
    }
}
