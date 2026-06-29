use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Documentation {
    pub definition: Option<String>,
    pub comments: Option<String>,
    pub requirements: Option<String>,
    pub examples: Option<Vec<String>>,
    pub short: Option<String>,
}

impl Documentation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn definition(mut self, definition: impl Into<String>) -> Self {
        self.definition = Some(definition.into());
        self
    }

    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }

    pub fn requirements(mut self, requirements: impl Into<String>) -> Self {
        self.requirements = Some(requirements.into());
        self
    }

    pub fn examples(mut self, examples: Vec<String>) -> Self {
        self.examples = Some(examples);
        self
    }

    pub fn short(mut self, short: impl Into<String>) -> Self {
        self.short = Some(short.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_documentation_new() {
        let doc = Documentation::new();
        assert!(doc.definition.is_none());
        assert!(doc.comments.is_none());
        assert!(doc.requirements.is_none());
        assert!(doc.examples.is_none());
        assert!(doc.short.is_none());
    }

    #[test]
    fn test_documentation_yaml_round_trip() {
        let doc = Documentation::new()
            .definition("A patient.")
            .short("Patient")
            .requirements("Must support US Core.");
        let yaml = serde_yaml::to_string(&doc).unwrap();
        let parsed: Documentation = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(doc, parsed);
    }

    fn document_with_examples() -> Documentation {
        Documentation::new()
            .definition("A patient resource.")
            .short("Patient")
            .comments("Demographics and other information.")
            .requirements("Must be validated.")
            .examples(vec!["Alice".to_string(), "Bob".to_string()])
    }

    #[test]
    fn test_documentation_full_yaml_round_trip() {
        let doc = document_with_examples();
        let yaml = serde_yaml::to_string(&doc).unwrap();
        let parsed: Documentation = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(doc, parsed);
    }
}
