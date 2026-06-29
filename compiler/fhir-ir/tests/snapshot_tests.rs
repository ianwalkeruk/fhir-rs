use fhir_ir::{Cardinality, Datatype, Field, Reference, Resource, Specification, Type};

const EXPECTED_PATIENT_YAML: &str = include_str!("patient.ir.yaml");

#[test]
fn test_patient_yaml_snapshot() {
    let patient = Resource::new("Patient").fields(vec![
        Field::new(
            "identifier",
            Type::datatype("Identifier"),
            Cardinality::Repeated,
        ),
        Field::new("active", Type::primitive("Boolean"), Cardinality::Optional),
    ]);

    let identifier = Datatype::new("Identifier").fields(vec![Field::new(
        "value",
        Type::primitive("string"),
        Cardinality::Optional,
    )]);

    let spec = Specification::new()
        .resources(vec![patient])
        .datatypes(vec![identifier]);

    let yaml = serde_yaml::to_string(&spec).unwrap();
    assert_eq!(yaml, EXPECTED_PATIENT_YAML);
}

#[test]
fn test_deterministic_serialization_invariant() {
    let spec = Specification::new().resources(vec![
        Resource::new("Patient").fields(vec![
            Field::new("active", Type::primitive("boolean"), Cardinality::Optional),
            Field::new("name", Type::datatype("HumanName"), Cardinality::Repeated),
        ]),
        Resource::new("Observation").fields(vec![
            Field::new("status", Type::primitive("code"), Cardinality::Required),
            Field::new(
                "subject",
                Type::reference(Reference::any()),
                Cardinality::Optional,
            ),
        ]),
    ]);

    let yaml1 = serde_yaml::to_string(&spec).unwrap();
    let yaml2 = serde_yaml::to_string(&spec).unwrap();
    assert_eq!(yaml1, yaml2, "YAML serialization must be deterministic");
}

#[test]
fn test_yaml_serialization_no_timestamps_invariant() {
    let spec =
        Specification::new().resources(vec![Resource::new("Patient").fields(vec![Field::new(
            "birthDate",
            Type::primitive("date"),
            Cardinality::Optional,
        )])]);

    let yaml = serde_yaml::to_string(&spec).unwrap();
    assert!(
        !yaml.contains("timestamp"),
        "YAML must not contain timestamps"
    );
    assert!(
        !yaml.contains("2026-"),
        "YAML must not contain machine timestamps"
    );
}

#[test]
fn test_yaml_serialization_no_machine_info_invariant() {
    let spec =
        Specification::new().resources(vec![Resource::new("Patient").fields(vec![Field::new(
            "identifier",
            Type::datatype("Identifier"),
            Cardinality::Repeated,
        )])]);

    let yaml = serde_yaml::to_string(&spec).unwrap();
    assert!(
        !yaml.contains(std::env::var("USER").unwrap_or_default().as_str()),
        "YAML must not contain machine-specific information"
    );
}
