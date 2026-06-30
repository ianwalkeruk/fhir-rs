use fhir_ir::ValueSet;

use crate::LoaderError;

pub trait ValueSetLoader {
    fn load_value_set(&self, name: &str) -> Result<Option<ValueSet>, LoaderError>;

    fn list_value_sets(&self) -> Result<Vec<String>, LoaderError>;
}
