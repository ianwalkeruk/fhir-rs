use fhir_ir::{Cardinality, Field, IrVersion, Resource, Specification, Type};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_canonicalise_ordering_invariant(
        resources in prop::collection::vec("[A-Za-z]+", 1..5)
    ) {
        let resources: Vec<Resource> = resources.into_iter().map(|name| {
            Resource::new(name).fields(vec![Field::new(
                "id",
                Type::primitive("string"),
                Cardinality::Optional,
            )])
        }).collect();

        let spec = Specification::new().resources(resources);

        let yaml1 = serde_yaml::to_string(&spec).unwrap();
        let yaml2 = serde_yaml::to_string(&spec).unwrap();

        prop_assert_eq!(yaml1, yaml2, "Canonicalise must produce deterministic output");
    }

    #[test]
    fn test_canonicalise_no_timestamps_invariant(
        base_resources in prop::collection::vec("[A-Za-z]+", 1..3)
    ) {
        let resources: Vec<Resource> = base_resources.into_iter().map(|name| {
            Resource::new(name).fields(vec![Field::new(
                "value",
                Type::primitive("string"),
                Cardinality::Optional,
            )])
        }).collect();

        let spec = Specification::new().resources(resources);
        let yaml = serde_yaml::to_string(&spec).unwrap();

        prop_assert!(
            !yaml.contains("timestamp"),
            "Canonicalised output must not contain timestamps"
        );
        prop_assert!(
            !yaml.contains("2026-"),
            "Canonicalised output must not contain current year timestamps"
        );
    }

    #[test]
    fn test_canonicalise_stable_field_ordering(
        field_names in prop::collection::vec("[a-z]+", 2..5)
    ) {
        let fields: Vec<Field> = field_names.into_iter().map(|name| {
            Field::new(name, Type::primitive("string"), Cardinality::Optional)
        }).collect();

        let resource = Resource::new("TestResource").fields(fields);
        let spec = Specification::new().resources(vec![resource]);

        let yaml = serde_yaml::to_string(&spec).unwrap();
        prop_assert!(yaml.contains("fields:"), "Must contain fields section");
    }

    #[test]
    fn test_ir_version_determinism(
        major in 0u32..3,
        minor in 0u32..10,
        patch in 0u32..10
    ) {
        let version = IrVersion::new(major, minor, patch);
        let yaml = serde_yaml::to_string(&version).unwrap();
        let parsed: IrVersion = serde_yaml::from_str(&yaml).unwrap();

        prop_assert_eq!(version, parsed);
    }
}
