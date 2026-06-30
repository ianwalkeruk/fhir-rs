use fhir_ir::CodeSystem;

use crate::LoaderError;

pub trait CodeSystemLoader {
    fn load_code_system(&self, name: &str) -> Result<Option<CodeSystem>, LoaderError>;

    fn list_code_systems(&self) -> Result<Vec<String>, LoaderError>;
}