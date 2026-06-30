use fhir_parser::{
    CodeSystemXml, ElementDefinitionXml, StructureDefinitionXml, ValueSetXml, XmlAttribute,
    XmlElement, XmlName, XmlSource, XmlTokenStream,
};

#[test]
fn test_xml_token_stream_new() {
    let stream: XmlTokenStream<String> = XmlTokenStream::new(vec!["a".to_string()]);
    assert_eq!(stream.into_tokens(), vec!["a".to_string()]);
}

#[test]
fn test_xml_token_stream_empty() {
    let stream: XmlTokenStream<String> = XmlTokenStream::empty();
    assert!(stream.into_tokens().is_empty());
}

#[test]
fn test_xml_token_stream_default() {
    let stream: XmlTokenStream<String> = XmlTokenStream::default();
    assert!(stream.into_tokens().is_empty());
}

#[test]
fn test_xml_name_new() {
    let name = XmlName::new("Patient");
    assert_eq!(name.local(), "Patient");
    assert!(name.prefix().is_none());
    assert!(name.namespace().is_none());
}

#[test]
fn test_xml_name_qualified() {
    let name = XmlName::qualified("fhir", "Patient");
    assert_eq!(name.local(), "Patient");
    assert_eq!(name.prefix(), Some("fhir"));
}

#[test]
fn test_xml_name_full() {
    let name = XmlName::full("fhir", "Patient", "http://hl7.org/fhir");
    assert_eq!(name.local(), "Patient");
    assert_eq!(name.prefix(), Some("fhir"));
    assert_eq!(name.namespace(), Some("http://hl7.org/fhir"));
}

#[test]
fn test_xml_attribute_local() {
    let attr = XmlAttribute::local("id", "123");
    assert_eq!(attr.value(), "123");
    assert_eq!(attr.name().local(), "id");
}

#[test]
fn test_xml_element_new() {
    let elem = XmlElement::new("Patient");
    assert_eq!(elem.name(), "Patient");
    assert!(elem.children().is_empty());
    assert!(elem.text().is_none());
}

#[test]
fn test_xml_source_default() {
    let source = XmlSource::default();
    assert!(source.structure_definitions.is_empty());
    assert!(source.value_sets.is_empty());
    assert!(source.code_systems.is_empty());
}

#[test]
fn test_structure_definition_xml_new() {
    let sd =
        StructureDefinitionXml::new("Patient", "http://hl7.org/fhir/StructureDefinition/Patient");
    assert_eq!(sd.id, "Patient");
    assert_eq!(sd.url, "http://hl7.org/fhir/StructureDefinition/Patient");
    assert!(sd.get_elements().is_empty());
}

#[test]
fn test_structure_definition_xml_with_elements() {
    let element = ElementDefinitionXml::default();
    let sd = StructureDefinitionXml::new("Patient", "url").elements(vec![element]);
    assert_eq!(sd.get_elements().len(), 1);
}

#[test]
fn test_value_set_xml_new() {
    let vs = ValueSetXml::new("AddressType", "http://hl7.org/fhir/ValueSet/address-type");
    assert_eq!(vs.id, "AddressType");
    assert_eq!(vs.url, "http://hl7.org/fhir/ValueSet/address-type");
}

#[test]
fn test_code_system_xml_new() {
    let cs = CodeSystemXml::new(
        "MaritalStatus",
        "http://hl7.org/fhir/CodeSystem/marital-status",
    );
    assert_eq!(cs.id, "MaritalStatus");
    assert_eq!(cs.url, "http://hl7.org/fhir/CodeSystem/marital-status");
}
