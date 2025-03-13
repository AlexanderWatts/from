#[derive(Debug, PartialEq)]
pub enum Proto {
    Element,
}

impl Proto {}

#[cfg(test)]
mod proto {
    use super::*;

    #[test]
    fn new() {
        let proto = Proto::Element;

        assert_eq!(Proto::Element, proto);
    }
}
