use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub targets: Vec<String>,
}

impl Reference {
    pub fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }

    pub fn any() -> Self {
        Self { targets: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_any() {
        let reference = Reference::any();
        assert!(reference.targets.is_empty());
    }

    #[test]
    fn test_reference_new() {
        let reference = Reference::new(vec!["Patient".to_string(), "Practitioner".to_string()]);
        assert_eq!(reference.targets.len(), 2);
        assert_eq!(reference.targets[0], "Patient");
    }

    #[test]
    fn test_reference_yaml_round_trip() {
        let reference = Reference::new(vec!["Patient".to_string(), "Organization".to_string()]);
        let yaml = serde_yaml::to_string(&reference).unwrap();
        let parsed: Reference = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(reference, parsed);
    }

    #[test]
    fn test_reference_any_yaml_round_trip() {
        let reference = Reference::any();
        let yaml = serde_yaml::to_string(&reference).unwrap();
        let parsed: Reference = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(reference, parsed);
    }
}
