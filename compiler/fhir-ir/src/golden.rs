#[cfg(test)]
mod tests {
    use crate::{
        Binding, BindingStrength, Cardinality, ChoiceType, Constraint, ConstraintSeverity,
        Datatype, Documentation, Field, IrVersion, Primitive, Reference, Resource, Specification,
        Type,
    };

    fn build_patient_specification() -> Specification {
        let patient = Resource::new("Patient")
            .documentation(
                Documentation::new()
                    .short("A patient")
                    .definition("Demographics and other administrative information."),
            )
            .fields(vec![
                Field::new(
                    "identifier",
                    Type::datatype("Identifier"),
                    Cardinality::Repeated,
                )
                .documentation(Documentation::new().short("Identifiers")),
                Field::new("active", Type::primitive("boolean"), Cardinality::Optional)
                    .documentation(
                        Documentation::new().short("Whether this patient's record is active"),
                    ),
            ])
            .constraints(vec![Constraint::new(
                "patient-active",
                ConstraintSeverity::Warning,
                "active.empty().not()",
                "Active flag should be set if known",
            )]);

        let identifier = Datatype::new("Identifier").fields(vec![Field::new(
            "value",
            Type::primitive("string"),
            Cardinality::Optional,
        )]);

        Specification::new()
            .resources(vec![patient])
            .datatypes(vec![identifier])
            .primitives(vec![
                Primitive::new("boolean"),
                Primitive::new("string"),
                Primitive::new("integer"),
                Primitive::new("decimal"),
                Primitive::new("uri"),
                Primitive::new("canonical"),
            ])
    }

    fn build_observation_specification() -> Specification {
        let observation = Resource::new("Observation")
            .documentation(
                Documentation::new()
                    .short("Observation")
                    .definition("Measurements and simple assertions."),
            )
            .fields(vec![
                Field::new(
                    "value",
                    Type::choice(ChoiceType::new(
                        "value",
                        vec![
                            Type::datatype("Quantity"),
                            Type::primitive("string"),
                            Type::primitive("boolean"),
                            Type::primitive("integer"),
                            Type::primitive("decimal"),
                            Type::datatype("CodeableConcept"),
                            Type::datatype("Range"),
                            Type::datatype("Ratio"),
                            Type::datatype("dateTime"),
                        ],
                    )),
                    Cardinality::Optional,
                )
                .documentation(Documentation::new().short("Actual result value")),
            ]);

        Specification::new().resources(vec![observation])
    }

    #[test]
    fn test_patient_specification_deterministic_yaml() {
        let spec = build_patient_specification();
        let yaml1 = serde_yaml::to_string(&spec).unwrap();
        let yaml2 = serde_yaml::to_string(&spec).unwrap();
        assert_eq!(yaml1, yaml2);
    }

    #[test]
    fn test_patient_specification_yaml_round_trip() {
        let spec = build_patient_specification();
        let yaml = serde_yaml::to_string(&spec).unwrap();
        let parsed: Specification = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(spec, parsed);
    }

    #[test]
    fn test_observation_specification_yaml_round_trip() {
        let spec = build_observation_specification();
        let yaml = serde_yaml::to_string(&spec).unwrap();
        let parsed: Specification = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(spec, parsed);
    }

    #[test]
    fn test_ir_version_in_specification() {
        let spec = Specification::new();
        assert_eq!(spec.version, IrVersion::CURRENT);
        assert_eq!(format!("{}", spec.version), "0.1.0");
    }

    #[test]
    fn test_reference_type_in_field() {
        let field = Field::new(
            "subject",
            Type::reference(Reference::new(vec![
                "Patient".to_string(),
                "Group".to_string(),
            ])),
            Cardinality::Optional,
        );
        let yaml = serde_yaml::to_string(&field).unwrap();
        let parsed: Field = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(field, parsed);
        if let Type::Reference(ref ref_) = parsed.type_ {
            assert_eq!(ref_.targets, vec!["Patient", "Group"]);
        } else {
            panic!("Expected Reference type");
        }
    }

    #[test]
    fn test_binding_on_resource() {
        let resource = Resource::new("Patient").bindings(vec![
            Binding::new(BindingStrength::Required)
                .value_set("http://hl7.org/fhir/ValueSet/patient-gender"),
        ]);
        let yaml = serde_yaml::to_string(&resource).unwrap();
        let parsed: Resource = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(resource, parsed);
    }
}
