#[cfg(test)]
mod end_to_end {
    use code_generation::CodeGenerator;
    use parser::Parser;
    use transformer::Transformer;

    #[test]
    fn create_elements_with_attributes() {
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

        let estree = Transformer.transform(&proto_root);
        let output = CodeGenerator::new().generate(&estree);

        assert_eq!(
            r#"function dom(target) {let div1 = element("div");let span2 = element("span");let t3 = literal("Main");attribute(div1, "id", "root");attribute(span2, "id", "nested");append(target, div1);append(div1, span2);append(span2, t3);}"#,
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

        let estree = Transformer.transform(&proto_root);
        let output = CodeGenerator::new().generate(&estree);

        assert_eq!(
            r#"function dom(target) {let div1 = element("div");let t2 = literal("Hello, 🌎!");let span3 = element("span");append(target, div1);append(div1, t2);append(div1, span3);}"#,
            output
        );
    }
}
