use serde::{de::DeserializeOwned, Serialize};

pub trait GoldenTest: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug {
    fn golden_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("GoldenTest types must serialize to YAML")
    }

    fn parse_golden_yaml(yaml: &str) -> Self {
        serde_yaml::from_str(yaml).expect("GoldenTest YAML must deserialize")
    }

    fn assert_golden_eq(&self, expected: &str) {
        let actual = self.golden_yaml();
        assert_eq!(actual, expected, "Golden test output mismatch");
    }

    fn assert_round_trip(&self) {
        let yaml = self.golden_yaml();
        let parsed = Self::parse_golden_yaml(&yaml);
        assert_eq!(*self, parsed, "Round-trip serialization failed");
    }

    fn assert_deterministic(&self, iterations: usize) {
        for _ in 0..iterations {
            let yaml1 = self.golden_yaml();
            let yaml2 = self.golden_yaml();
            assert_eq!(yaml1, yaml2, "Serialization must be deterministic");
        }
    }
}

pub fn assert_deterministic_serialization<T: Serialize>(value: &T) -> String {
    let yaml1 = serde_yaml::to_string(value).expect("Value must serialize to YAML");
    let yaml2 = serde_yaml::to_string(value).expect("Value must serialize to YAML");
    assert_eq!(yaml1, yaml2, "Serialization must be deterministic (INV-001)");
    yaml1
}

pub fn assert_round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let yaml = serde_yaml::to_string(value).expect("Value must serialize to YAML");
    let parsed: T = serde_yaml::from_str(&yaml).expect("YAML must deserialize");
    assert_eq!(*value, parsed, "Round-trip serialization failed");
}

pub fn assert_no_timestamps(yaml: &str) {
    assert!(!yaml.contains("timestamp"), "YAML must not contain timestamps");
    assert!(!yaml.contains("2026-"), "YAML must not contain machine timestamps");
}

pub fn assert_no_machine_info(yaml: &str) {
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            assert!(!yaml.contains(user.as_str()), "YAML must not contain machine-specific information");
        }
    }
}