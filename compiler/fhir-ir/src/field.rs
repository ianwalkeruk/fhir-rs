use serde::{Deserialize, Serialize};

use crate::{Cardinality, Constraint, Documentation, Type};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub type_: Type,
    pub cardinality: Cardinality,
    pub documentation: Option<Documentation>,
    pub summary: bool,
    pub modifier: bool,
    pub must_support: bool,
    pub constraints: Vec<Constraint>,
}

impl Field {
    pub fn new(name: impl Into<String>, type_: Type, cardinality: Cardinality) -> Self {
        Self {
            name: name.into(),
            type_,
            cardinality,
            documentation: None,
            summary: false,
            modifier: false,
            must_support: false,
            constraints: Vec::new(),
        }
    }

    pub fn documentation(mut self, documentation: Documentation) -> Self {
        self.documentation = Some(documentation);
        self
    }

    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = summary;
        self
    }

    pub fn modifier(mut self, modifier: bool) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn must_support(mut self, must_support: bool) -> Self {
        self.must_support = must_support;
        self
    }

    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_field() -> Field {
        Field::new("active", Type::primitive("boolean"), Cardinality::Optional)
            .documentation(Documentation::new().short("Active flag"))
    }

    #[test]
    fn test_field_new() {
        let field = sample_field();
        assert_eq!(field.name, "active");
        assert_eq!(field.cardinality, Cardinality::Optional);
        assert!(!field.modifier);
        assert!(!field.must_support);
    }

    #[test]
    fn test_field_yaml_round_trip() {
        let field = sample_field();
        let yaml = serde_yaml::to_string(&field).unwrap();
        let parsed: Field = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(field, parsed);
    }
}
