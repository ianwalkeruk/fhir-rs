use fhir_parser::{
    BindingStrengthXml, ConstraintSeverityXml, Derivation, ParseError, PublicationStatus,
    StructureDefinitionKind,
};
use std::path::PathBuf;

#[test]
fn test_parse_error_io() {
    let err = ParseError::io(PathBuf::from("test.xml"), "file not found");
    assert!(err.to_string().contains("test.xml"));
    assert!(err.to_string().contains("file not found"));
}

#[test]
fn test_parse_error_xml_syntax() {
    let err = ParseError::xml_syntax(10, 5, "unexpected token");
    let msg = err.to_string();
    assert!(msg.contains("line 10"));
    assert!(msg.contains("column 5"));
    assert!(msg.contains("unexpected token"));
}

#[test]
fn test_parse_error_unexpected_element() {
    let err = ParseError::unexpected_element("StructureDefinition", "Element");
    let msg = err.to_string();
    assert!(msg.contains("StructureDefinition"));
    assert!(msg.contains("Element"));
}

#[test]
fn test_parse_error_missing_element() {
    let err = ParseError::missing_element("element");
    assert!(err.to_string().contains("element"));
}

#[test]
fn test_parse_error_invalid_attribute() {
    let err = ParseError::invalid_attribute("StructureDefinition", "url", "must be absolute");
    let msg = err.to_string();
    assert!(msg.contains("StructureDefinition"));
    assert!(msg.contains("url"));
    assert!(msg.contains("must be absolute"));
}

#[test]
fn test_parse_error_invalid_content() {
    let err = ParseError::invalid_content("value out of range");
    assert!(err.to_string().contains("value out of range"));
}

#[test]
fn test_parse_error_invalid_structure_definition() {
    let err = ParseError::invalid_structure_definition("Patient", "missing snapshot");
    let msg = err.to_string();
    assert!(msg.contains("Patient"));
    assert!(msg.contains("missing snapshot"));
}

#[test]
fn test_parse_error_invalid_value_set() {
    let err = ParseError::invalid_value_set("AddressType", "missing concepts");
    let msg = err.to_string();
    assert!(msg.contains("AddressType"));
    assert!(msg.contains("missing concepts"));
}

#[test]
fn test_parse_error_invalid_code_system() {
    let err = ParseError::invalid_code_system("MaritalStatus", "missing concepts");
    let msg = err.to_string();
    assert!(msg.contains("MaritalStatus"));
    assert!(msg.contains("missing concepts"));
}

#[test]
fn test_parse_error_unsupported_version() {
    let err = ParseError::UnsupportedVersion("R3".to_string());
    assert!(err.to_string().contains("R3"));
}

#[test]
fn test_parse_error_no_panic_on_invalid_input() {
    let result = std::panic::catch_unwind(|| {
        let _err = ParseError::EmptyDocument;
        let _err = ParseError::MultipleRootElements;
    });
    assert!(result.is_ok());
}

#[test]
fn test_publication_status_values() {
    let _ = PublicationStatus::Draft;
    let _ = PublicationStatus::Active;
    let _ = PublicationStatus::Retired;
    let _ = PublicationStatus::Unknown;
}

#[test]
fn test_structure_definition_kind_values() {
    let _ = StructureDefinitionKind::Resource;
    let _ = StructureDefinitionKind::Datatype;
    let _ = StructureDefinitionKind::Primitive;
    let _ = StructureDefinitionKind::ComplexType;
    let _ = StructureDefinitionKind::NotApplicable;
}

#[test]
fn test_derivation_values() {
    let _ = Derivation::Specialization;
    let _ = Derivation::Constraint;
}

#[test]
fn test_constraint_severity_xml_values() {
    let _ = ConstraintSeverityXml::Error;
    let _ = ConstraintSeverityXml::Warning;
    let _ = ConstraintSeverityXml::Information;
}

#[test]
fn test_binding_strength_xml_values() {
    let _ = BindingStrengthXml::Required;
    let _ = BindingStrengthXml::Extensible;
    let _ = BindingStrengthXml::Preferred;
    let _ = BindingStrengthXml::Example;
}
