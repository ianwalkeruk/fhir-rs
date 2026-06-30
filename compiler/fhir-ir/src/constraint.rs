use serde::{Deserialize, Serialize};

use crate::GoldenTest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub id: String,
    pub severity: ConstraintSeverity,
    pub expression: String,
    pub human_description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    Error,
    Warning,
    Information,
}

impl Constraint {
    pub fn new(
        id: impl Into<String>,
        severity: ConstraintSeverity,
        expression: impl Into<String>,
        human_description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            expression: expression.into(),
            human_description: human_description.into(),
        }
    }
}

impl GoldenTest for Constraint {}

impl GoldenTest for ConstraintSeverity {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_new() {
        let constraint = Constraint::new(
            "constraint-1",
            ConstraintSeverity::Error,
            "patient.name.empty()",
            "Name must not be empty",
        );
        assert_eq!(constraint.id, "constraint-1");
        assert_eq!(constraint.severity, ConstraintSeverity::Error);
    }

    #[test]
    fn test_constraint_yaml_round_trip() {
        let constraint = Constraint::new(
            "cq-name-empty",
            ConstraintSeverity::Warning,
            "name.given.count() > 0",
            "Given name should be present",
        );
        let yaml = serde_yaml::to_string(&constraint).unwrap();
        let parsed: Constraint = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(constraint, parsed);
    }

    #[test]
    fn test_constraint_severity_yaml_round_trip() {
        for severity in [
            ConstraintSeverity::Error,
            ConstraintSeverity::Warning,
            ConstraintSeverity::Information,
        ] {
            let yaml = serde_yaml::to_string(&severity).unwrap();
            let parsed: ConstraintSeverity = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(severity, parsed);
        }
    }
}
