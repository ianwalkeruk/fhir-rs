use serde::{Deserialize, Serialize};

use crate::GoldenTest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cardinality {
    Optional,
    Required,
    Repeated,
    RequiredRepeated,
}

impl GoldenTest for Cardinality {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardinality_variants() {
        let variants = [
            Cardinality::Optional,
            Cardinality::Required,
            Cardinality::Repeated,
            Cardinality::RequiredRepeated,
        ];
        assert_eq!(variants.len(), 4);
    }

    #[test]
    fn test_cardinality_yaml_round_trip() {
        for card in [
            Cardinality::Optional,
            Cardinality::Required,
            Cardinality::Repeated,
            Cardinality::RequiredRepeated,
        ] {
            let yaml = serde_yaml::to_string(&card).unwrap();
            let parsed: Cardinality = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(card, parsed);
        }
    }
}
