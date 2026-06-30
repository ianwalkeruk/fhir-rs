use serde::{Deserialize, Serialize};

use crate::{Binding, Constraint, Documentation, Field, GoldenTest};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datatype {
    pub name: String,
    pub documentation: Option<Documentation>,
    pub fields: Vec<Field>,
    pub constraints: Vec<Constraint>,
    pub bindings: Vec<Binding>,
}

impl Datatype {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            documentation: None,
            fields: Vec::new(),
            constraints: Vec::new(),
            bindings: Vec::new(),
        }
    }

    pub fn documentation(mut self, documentation: Documentation) -> Self {
        self.documentation = Some(documentation);
        self
    }

    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn bindings(mut self, bindings: Vec<Binding>) -> Self {
        self.bindings = bindings;
        self
    }
}

impl GoldenTest for Datatype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, Type};

    #[test]
    fn test_datatype_new() {
        let dt = Datatype::new("Identifier");
        assert_eq!(dt.name, "Identifier");
        assert!(dt.fields.is_empty());
    }

    #[test]
    fn test_datatype_yaml_round_trip() {
        let dt = Datatype::new("HumanName").fields(vec![Field::new(
            "family",
            Type::primitive("string"),
            Cardinality::Optional,
        )]);
        let yaml = serde_yaml::to_string(&dt).unwrap();
        let parsed: Datatype = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(dt, parsed);
    }
}
