use serde::{Deserialize, Serialize};

use crate::Documentation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    pub strength: BindingStrength,
    pub value_set: Option<String>,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingStrength {
    Required,
    Preferred,
    Extensible,
    Example,
}

impl Binding {
    pub fn new(strength: BindingStrength) -> Self {
        Self {
            strength,
            value_set: None,
            documentation: None,
        }
    }

    pub fn value_set(mut self, value_set: impl Into<String>) -> Self {
        self.value_set = Some(value_set.into());
        self
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
    fn test_binding_new() {
        let binding = Binding::new(BindingStrength::Required);
        assert_eq!(binding.strength, BindingStrength::Required);
        assert!(binding.value_set.is_none());
        assert!(binding.documentation.is_none());
    }

    #[test]
    fn test_binding_yaml_round_trip() {
        let binding = Binding::new(BindingStrength::Extensible)
            .value_set("http://hl7.org/fhir/ValueSet/address-type");
        let yaml = serde_yaml::to_string(&binding).unwrap();
        let parsed: Binding = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(binding, parsed);
    }

    #[test]
    fn test_binding_strength_yaml_round_trip() {
        for strength in [
            BindingStrength::Required,
            BindingStrength::Preferred,
            BindingStrength::Extensible,
            BindingStrength::Example,
        ] {
            let yaml = serde_yaml::to_string(&strength).unwrap();
            let parsed: BindingStrength = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(strength, parsed);
        }
    }
}
