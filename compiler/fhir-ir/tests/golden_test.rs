use fhir_ir::{Cardinality, Field, Resource, Specification, Type};
use fhir_ir::golden_test::{assert_deterministic_serialization, assert_round_trip, assert_no_timestamps, assert_no_machine_info};

#[test]
fn test_golden_test_trait_required_for_all_ir_types() {
    // GOLDEN-TEST-REQ-001: All IR types must implement GoldenTest trait
    // This test will fail until the trait is implemented for all IR types
    // 
    // Required trait methods:
    // - golden_yaml() -> String
    // - parse_golden_yaml(yaml: &str) -> Self
    // - assert_golden_eq(expected: &str)
    // - assert_round_trip()
    // - assert_deterministic(iterations: usize)
    //
    // Once implemented, this test should compile and pass:
    // let patient = Resource::new("Patient").fields(vec![...]);
    // patient.assert_round_trip();
    // patient.assert_deterministic(10);
    
    assert!(false, "GoldenTest trait must be implemented for all IR types. See test_golden_test_trait_required_for_all_ir_types for requirements.");
}

#[test]
fn test_assert_deterministic_serialization() {
    let resource = Resource::new("Patient").fields(vec![
        Field::new("active", Type::primitive("boolean"), Cardinality::Optional),
    ]);
    
    let spec = Specification::new().resources(vec![resource]);
    let yaml = assert_deterministic_serialization(&spec);
    assert!(!yaml.is_empty());
}

#[test]
fn test_assert_round_trip_specification() {
    let spec = Specification::new().resources(vec![
        Resource::new("Observation").fields(vec![
            Field::new("status", Type::primitive("code"), Cardinality::Required),
        ]),
    ]);
    
    assert_round_trip(&spec);
}

#[test]
fn test_assert_no_timestamps() {
    let spec = Specification::new().resources(vec![
        Resource::new("Patient").fields(vec![
            Field::new("birthDate", Type::primitive("date"), Cardinality::Optional),
        ]),
    ]);
    
    let yaml = serde_yaml::to_string(&spec).unwrap();
    assert_no_timestamps(&yaml);
}

#[test]
fn test_assert_no_machine_info() {
    let spec = Specification::new().resources(vec![
        Resource::new("Patient").fields(vec![
            Field::new("identifier", Type::datatype("Identifier"), Cardinality::Repeated),
        ]),
    ]);
    
    let yaml = serde_yaml::to_string(&spec).unwrap();
    assert_no_machine_info(&yaml);
}

// Property tests for IR invariants

proptest::proptest! {
    #[test]
    fn test_deterministic_serialization_proptest(
        base_resources in proptest::collection::vec("[A-Za-z]+", 1..5)
    ) {
        let resources: Vec<Resource> = base_resources.into_iter().map(|name| {
            Resource::new(name.clone()).fields(vec![
                Field::new("id", Type::primitive("string"), Cardinality::Optional),
            ])
        }).collect();

        let spec = Specification::new().resources(resources);
        let yaml = serde_yaml::to_string(&spec).unwrap();
        let parsed: Specification = serde_yaml::from_str(&yaml).unwrap();
        
        proptest::prop_assert_eq!(spec, parsed);
    }

    #[test]
    fn test_no_timestamps_in_variable_spec(
        resource_names in proptest::collection::vec("[A-Za-z]+", 1..3)
    ) {
        let resources: Vec<Resource> = resource_names.into_iter().map(|name| {
            Resource::new(name).fields(vec![Field::new(
                "value",
                Type::primitive("string"),
                Cardinality::Optional,
            )])
        }).collect();

        let spec = Specification::new().resources(resources);
        let yaml = serde_yaml::to_string(&spec).unwrap();
        
        proptest::prop_assert!(!yaml.contains("timestamp"));
        proptest::prop_assert!(!yaml.contains("2026-"));
    }

    #[test]
    fn test_serialization_round_trip(
        field_count in 1usize..5
    ) {
        use fhir_ir::Datatype;
        
        let fields: Vec<Field> = (0..field_count).map(|i| {
            Field::new(format!("field{}", i), Type::primitive("boolean"), Cardinality::Optional)
        }).collect();

        let datatype = Datatype::new("TestType").fields(fields);
        let yaml = serde_yaml::to_string(&datatype).unwrap();
        let parsed: Datatype = serde_yaml::from_str(&yaml).unwrap();
        
        proptest::prop_assert_eq!(datatype, parsed);
    }
}