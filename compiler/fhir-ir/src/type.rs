use serde::{Deserialize, Serialize};

use crate::{ChoiceType, Reference};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Primitive(String),
    Datatype(String),
    Resource(String),
    Choice(ChoiceType),
    Reference(Reference),
}

impl Type {
    pub fn primitive(name: impl Into<String>) -> Self {
        Self::Primitive(name.into())
    }

    pub fn datatype(name: impl Into<String>) -> Self {
        Self::Datatype(name.into())
    }

    pub fn resource(name: impl Into<String>) -> Self {
        Self::Resource(name.into())
    }

    pub fn choice(choice_type: ChoiceType) -> Self {
        Self::Choice(choice_type)
    }

    pub fn reference(reference: Reference) -> Self {
        Self::Reference(reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_primitive() {
        let t = Type::primitive("string");
        assert!(matches!(t, Type::Primitive(ref s) if s == "string"));
    }

    #[test]
    fn test_type_yaml_round_trip() {
        let t = Type::Resource("Patient".to_string());
        let yaml = serde_yaml::to_string(&t).unwrap();
        let parsed: Type = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(t, parsed);
    }

    #[test]
    fn test_type_choice_yaml_round_trip() {
        let t = Type::choice(ChoiceType::new(
            "value",
            vec![Type::primitive("Quantity"), Type::primitive("string")],
        ));
        let yaml = serde_yaml::to_string(&t).unwrap();
        let parsed: Type = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(t, parsed);
    }
}
