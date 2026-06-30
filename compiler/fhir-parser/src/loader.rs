use fhir_ir::Specification;

use crate::LoaderError;

pub trait Loader {
    fn load(&self) -> Result<Specification, LoaderError>;
}