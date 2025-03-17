#[cfg(test)]
mod end_to_end {
    use code_generation::CodeGenerator;
    use parser::Parser;
    use transpiler::Transpiler;

    #[test]
    fn create_dom_tree() {
        let input = r#"div {span {} div {}}"#;

        let parser = Parser::new(input);

        let proto_root = match parser.parse() {
            Ok(proto) => proto,
            Err(_) => return eprintln!("Parser error"),
        };

        let js_root = Transpiler.transpile(&proto_root);
        let output = CodeGenerator::new().generate(&js_root);

        assert_eq!(
            r#"function dom() {return element("div", element("span"), element("div"))}"#,
            output
        );
    }
}
