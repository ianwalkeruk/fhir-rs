use serde::{Deserialize, Serialize};

use crate::Documentation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Primitive {
    pub name: String,
    pub documentation: Option<Documentation>,
}

impl Primitive {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            documentation: None,
        }
    }

    pub fn documentation(mut self, documentation: Documentation) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_new() {
        let primitive = Primitive::new("string");
        assert_eq!(primitive.name, "string");
        assert!(primitive.documentation.is_none());
    }

    #[test]
    fn test_primitive_yaml_round_trip() {
        let primitive = Primitive::new("integer")
            .documentation(Documentation::new().definition("An integer value."));
        let yaml = serde_yaml::to_string(&primitive).unwrap();
        let parsed: Primitive = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(primitive, parsed);
    }
}
