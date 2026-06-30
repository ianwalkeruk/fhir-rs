use fhir_ir::{Datatype, Resource};

use crate::LoaderError;

pub trait ResourceLoader {
    fn load_resource(&self, name: &str) -> Result<Option<Resource>, LoaderError>;

    fn load_datatype(&self, name: &str) -> Result<Option<Datatype>, LoaderError>;

    fn list_resources(&self) -> Result<Vec<String>, LoaderError>;

    fn list_datatypes(&self) -> Result<Vec<String>, LoaderError>;
}
