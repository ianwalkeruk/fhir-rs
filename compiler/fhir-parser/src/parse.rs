use fhir_ir::Specification;

use crate::ParseError;

pub trait Parse {
    fn parse(&self) -> Result<Specification, ParseError>;
}
