use serde::{Deserialize, Serialize};

use crate::{Datatype, Documentation, GoldenTest, IrVersion, Primitive, Resource};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specification {
    #[serde(rename = "ir_version")]
    pub version: IrVersion,
    pub resources: Vec<Resource>,
    pub datatypes: Vec<Datatype>,
    pub primitives: Vec<Primitive>,
    pub value_sets: Vec<ValueSet>,
    pub code_systems: Vec<CodeSystem>,
    pub search_parameters: Vec<SearchParameter>,
}

impl Default for Specification {
    fn default() -> Self {
        Self {
            version: IrVersion::CURRENT,
            resources: Vec::new(),
            datatypes: Vec::new(),
            primitives: Vec::new(),
            value_sets: Vec::new(),
            code_systems: Vec::new(),
            search_parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueSet {
    pub name: String,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSystem {
    pub name: String,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchParameter {
    pub name: String,
    pub base: String,
    pub type_: SearchParameterType,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchParameterType {
    Number,
    Date,
    String,
    Boolean,
    Composite,
    Quantity,
    Uri,
    Reference,
    Choice,
    Coding,
}

impl Specification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resources(mut self, resources: Vec<Resource>) -> Self {
        self.resources = resources;
        self
    }

    pub fn datatypes(mut self, datatypes: Vec<Datatype>) -> Self {
        self.datatypes = datatypes;
        self
    }

    pub fn primitives(mut self, primitives: Vec<Primitive>) -> Self {
        self.primitives = primitives;
        self
    }

    pub fn value_sets(mut self, value_sets: Vec<ValueSet>) -> Self {
        self.value_sets = value_sets;
        self
    }

    pub fn code_systems(mut self, code_systems: Vec<CodeSystem>) -> Self {
        self.code_systems = code_systems;
        self
    }

    pub fn search_parameters(mut self, search_parameters: Vec<SearchParameter>) -> Self {
        self.search_parameters = search_parameters;
        self
    }
}

impl GoldenTest for Specification {}

impl GoldenTest for ValueSet {}

impl GoldenTest for CodeSystem {}

impl GoldenTest for SearchParameter {}

impl GoldenTest for SearchParameterType {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, Field, Type};

    #[test]
    fn test_specification_new() {
        let spec = Specification::new();
        assert_eq!(spec.version, IrVersion::CURRENT);
        assert!(spec.resources.is_empty());
        assert!(spec.datatypes.is_empty());
    }

    #[test]
    fn test_specification_yaml_round_trip() {
        let spec = Specification::new().resources(vec![Resource::new("Patient").fields(vec![
            Field::new("active", Type::primitive("boolean"), Cardinality::Optional),
        ])]);
        let yaml = serde_yaml::to_string(&spec).unwrap();
        let parsed: Specification = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(spec, parsed);
    }

    #[test]
    fn test_value_set_yaml_round_trip() {
        let vs = ValueSet {
            name: "AddressType".to_string(),
            documentation: Some(Documentation::new().short("Address types")),
        };
        let yaml = serde_yaml::to_string(&vs).unwrap();
        let parsed: ValueSet = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(vs, parsed);
    }

    #[test]
    fn test_code_system_yaml_round_trip() {
        let cs = CodeSystem {
            name: "MaritalStatus".to_string(),
            documentation: Some(Documentation::new().short("Marital status codes")),
        };
        let yaml = serde_yaml::to_string(&cs).unwrap();
        let parsed: CodeSystem = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(cs, parsed);
    }
}
