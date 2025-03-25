#[cfg(test)]
mod end_to_end {
    use code_generation::CodeGenerator;
    use parser::Parser;
    use transpiler::Transpiler;

    #[test]
    fn elements_with_attrs() {
        let input = r#"
            div {
                @id="root"

                span {
                    @id="nested"
                    "Main"
                }
            }
        "#;

        let mut parser = Parser::new(input);

        let proto_root = match parser.parse() {
            Ok(proto) => proto,
            Err(_) => return eprintln!("Parser error"),
        };

        let js_root = Transpiler.transpile(&proto_root);
        let output = CodeGenerator::new().generate(&js_root);

        assert_eq!(
            r#"function dom() {return element("div", {id: "root"}, element("span", {id: "nested"}, literal("Main")))}"#,
            output
        );
    }

    #[test]
    fn create_dom_tree() {
        let input = r#"div {"Hello, 🌎!" span {} }"#;

        let mut parser = Parser::new(input);

        let proto_root = match parser.parse() {
            Ok(proto) => proto,
            Err(_) => return eprintln!("Parser error"),
        };

        let js_root = Transpiler.transpile(&proto_root);
        let output = CodeGenerator::new().generate(&js_root);

        assert_eq!(
            r#"function dom() {return element("div", {}, literal("Hello, 🌎!"), element("span", {}))}"#,
            output
        );
    }
}
