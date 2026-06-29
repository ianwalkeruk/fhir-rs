use serde::{Deserialize, Serialize};

use crate::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceType {
    pub base_name: String,
    pub variants: Vec<Type>,
}

impl ChoiceType {
    pub fn new(base_name: impl Into<String>, variants: Vec<Type>) -> Self {
        Self {
            base_name: base_name.into(),
            variants,
        }
    }

    pub fn variants(&self) -> &[Type] {
        &self.variants
    }

    pub fn base_name(&self) -> &str {
        &self.base_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_choice_type() -> ChoiceType {
        ChoiceType::new(
            "value",
            vec![
                Type::primitive("Quantity"),
                Type::primitive("string"),
                Type::primitive("boolean"),
            ],
        )
    }

    #[test]
    fn test_choice_type_new() {
        let choice = sample_choice_type();
        assert_eq!(choice.base_name(), "value");
        assert_eq!(choice.variants().len(), 3);
    }

    #[test]
    fn test_choice_type_yaml_round_trip() {
        let choice = sample_choice_type();
        let yaml = serde_yaml::to_string(&choice).unwrap();
        let parsed: ChoiceType = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(choice, parsed);
    }
}
