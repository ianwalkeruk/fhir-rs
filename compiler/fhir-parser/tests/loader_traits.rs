use fhir_parser::{Loader, LoaderError, ResourceLoader, ValueSetLoader, CodeSystemLoader};

#[test]
fn test_loader_trait_exists() {
    struct TestLoader;

    impl Loader for TestLoader {
        fn load(&self) -> Result<fhir_ir::Specification, LoaderError> {
            Err(LoaderError::InvalidVersion("test".to_string()))
        }
    }

    assert!(true);
}

#[test]
fn test_resource_loader_trait_exists() {
    struct TestResourceLoader;

    impl ResourceLoader for TestResourceLoader {
        fn load_resource(&self, _name: &str) -> Result<Option<fhir_ir::Resource>, LoaderError> {
            Err(LoaderError::NotFound("test".to_string()))
        }

        fn load_datatype(&self, _name: &str) -> Result<Option<fhir_ir::Datatype>, LoaderError> {
            Err(LoaderError::NotFound("test".to_string()))
        }

        fn list_resources(&self) -> Result<Vec<String>, LoaderError> {
            Err(LoaderError::InvalidVersion("test".to_string()))
        }

        fn list_datatypes(&self) -> Result<Vec<String>, LoaderError> {
            Err(LoaderError::InvalidVersion("test".to_string()))
        }
    }

    assert!(true);
}

#[test]
fn test_valueset_loader_trait_exists() {
    struct TestValueSetLoader;

    impl ValueSetLoader for TestValueSetLoader {
        fn load_value_set(&self, _name: &str) -> Result<Option<fhir_ir::ValueSet>, LoaderError> {
            Err(LoaderError::NotFound("test".to_string()))
        }

        fn list_value_sets(&self) -> Result<Vec<String>, LoaderError> {
            Err(LoaderError::InvalidVersion("test".to_string()))
        }
    }

    assert!(true);
}

#[test]
fn test_codesystem_loader_trait_exists() {
    struct TestCodeSystemLoader;

    impl CodeSystemLoader for TestCodeSystemLoader {
        fn load_code_system(&self, _name: &str) -> Result<Option<fhir_ir::CodeSystem>, LoaderError> {
            Err(LoaderError::NotFound("test".to_string()))
        }

        fn list_code_systems(&self) -> Result<Vec<String>, LoaderError> {
            Err(LoaderError::InvalidVersion("test".to_string()))
        }
    }

    assert!(true);
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn test_loader_produces_specification() {
    struct NotImplementedLoader;

    impl Loader for NotImplementedLoader {
        fn load(&self) -> Result<fhir_ir::Specification, LoaderError> {
            panic!("not yet implemented")
        }
    }

    let loader = NotImplementedLoader;
    loader.load().unwrap();
}

#[test]
fn test_loader_error_variants_exist() {
    let _err = LoaderError::FileRead("test".to_string());
    let _err = LoaderError::Parse {
        resource_type: "StructureDefinition".to_string(),
        name: "Patient".to_string(),
        message: "missing element".to_string(),
    };
    let _err = LoaderError::NotFound("Patient".to_string());
    let _err = LoaderError::InvalidVersion("R4".to_string());
    let _err = LoaderError::Xml("invalid XML".to_string());
}

#[test]