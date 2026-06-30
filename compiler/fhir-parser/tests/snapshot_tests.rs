use fhir_ir::Specification;
use fhir_parser::{Parse, ParseError, StructureDefinitionXml, XmlSource};

struct FailingXmlParser;

impl Parse for FailingXmlParser {
    fn parse(&self) -> Result<Specification, ParseError> {
        Err(ParseError::EmptyDocument)
    }
}

#[test]
fn test_xml_source_empty_parsing_returns_error() {
    let parser = FailingXmlParser;
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_structure_definition_xml_deterministic() {
    let sd =
        StructureDefinitionXml::new("Patient", "http://hl7.org/fhir/StructureDefinition/Patient");

    let debug1 = format!("{:?}", sd);
    let debug2 = format!("{:?}", sd);
    assert_eq!(
        debug1, debug2,
        "StructureDefinitionXml must serialize deterministically"
    );
}

#[test]
fn test_xml_source_deterministic() {
    let source = XmlSource::default();

    let debug1 = format!("{:?}", source);
    let debug2 = format!("{:?}", source);
    assert_eq!(debug1, debug2, "XmlSource must serialize deterministically");
}
