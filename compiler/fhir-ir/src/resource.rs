use serde::{Deserialize, Serialize};

use crate::{Binding, Constraint, Documentation, Field};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub documentation: Option<Documentation>,
    pub fields: Vec<Field>,
    pub constraints: Vec<Constraint>,
    pub bindings: Vec<Binding>,
}

impl Resource {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, Type};

    #[test]
    fn test_resource_new() {
        let resource = Resource::new("Patient");
        assert_eq!(resource.name, "Patient");
        assert!(resource.fields.is_empty());
    }

    #[test]
    fn test_resource_yaml_round_trip() {
        let resource = Resource::new("Patient")
            .documentation(Documentation::new().short("A patient"))
            .fields(vec![
                Field::new("active", Type::primitive("boolean"), Cardinality::Optional),
                Field::new(
                    "identifier",
                    Type::datatype("Identifier"),
                    Cardinality::Repeated,
                ),
            ]);
        let yaml = serde_yaml::to_string(&resource).unwrap();
        let parsed: Resource = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(resource, parsed);
    }
}
