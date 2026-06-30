use fhir_ir::Specification;
use fhir_parser::{Parse, ParseError};

struct NotImplementedParser;

impl Parse for NotImplementedParser {
    fn parse(&self) -> Result<Specification, ParseError> {
        panic!("not yet implemented - XML parser interface defined but no implementation")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn test_xml_parser_not_implemented() {
    let parser = NotImplementedParser;
    parser.parse().unwrap();
}

#[test]
fn test_parse_trait_exists() {
    struct TestXmlParser;

    #[allow(dead_code)]
    impl Parse for TestXmlParser {
        fn parse(&self) -> Result<Specification, ParseError> {
            Err(ParseError::EmptyDocument)
        }
    }
}

#[test]
fn test_parse_trait_returns_initial_ir() {
    struct MockParser;

    impl Parse for MockParser {
        fn parse(&self) -> Result<Specification, ParseError> {
            Ok(Specification::new())
        }
    }

    let parser = MockParser;
    let result = parser.parse();
    assert!(result.is_ok());
    let spec = result.unwrap();
    assert!(spec.resources.is_empty());
}
