pub use fhir_ir::*;

pub mod schema {
    pub use fhir_ir::binding::{Binding, BindingStrength};
    pub use fhir_ir::cardinality::Cardinality;
    pub use fhir_ir::choice_type::ChoiceType;
    pub use fhir_ir::constraint::{Constraint, ConstraintSeverity};
    pub use fhir_ir::datatype::Datatype;
    pub use fhir_ir::documentation::Documentation;
    pub use fhir_ir::field::Field;
    pub use fhir_ir::ir::IrVersion;
    pub use fhir_ir::primitive::Primitive;
    pub use fhir_ir::reference::Reference;
    pub use fhir_ir::resource::Resource;
    pub use fhir_ir::specification::{
        CodeSystem, SearchParameter, SearchParameterType, Specification, ValueSet,
    };
    pub use fhir_ir::r#type::Type;
}
