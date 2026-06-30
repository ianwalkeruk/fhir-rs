use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlTokenStream<T> {
    tokens: Vec<T>,
}

impl<T> XmlTokenStream<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self { tokens }
    }

    pub fn empty() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn into_tokens(self) -> Vec<T> {
        self.tokens
    }
}

impl<T> Default for XmlTokenStream<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlEvent {
    StartElement {
        name: XmlName,
        attributes: Vec<XmlAttribute>,
    },
    EndElement {
        name: XmlName,
    },
    Text {
        content: String,
    },
    CData {
        content: String,
    },
    Comment {
        content: String,
    },
    ProcessingInstruction {
        target: String,
        content: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlName {
    prefix: Option<String>,
    local: String,
    namespace: Option<String>,
}

impl XmlName {
    pub fn new(local: impl Into<String>) -> Self {
        Self {
            prefix: None,
            local: local.into(),
            namespace: None,
        }
    }

    pub fn qualified(prefix: impl Into<String>, local: impl Into<String>) -> Self {
        Self {
            prefix: Some(prefix.into()),
            local: local.into(),
            namespace: None,
        }
    }

    pub fn full(
        prefix: impl Into<String>,
        local: impl Into<String>,
        namespace: impl Into<String>,
    ) -> Self {
        Self {
            prefix: Some(prefix.into()),
            local: local.into(),
            namespace: Some(namespace.into()),
        }
    }

    pub fn local(&self) -> &str {
        &self.local
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}

impl fmt::Display for XmlName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.prefix {
            Some(p) => write!(f, "{}:{}", p, self.local),
            None => write!(f, "{}", self.local),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlAttribute {
    name: XmlName,
    value: String,
}

impl XmlAttribute {
    pub fn new(name: XmlName, value: impl Into<String>) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }

    pub fn local(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: XmlName::new(name),
            value: value.into(),
        }
    }

    pub fn qualified(
        _prefix: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            name: XmlName::new(name),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &XmlName {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlElement {
    name: String,
    attributes: Vec<String>,
    children: Vec<XmlElement>,
    text: Option<String>,
}

impl XmlElement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            text: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    pub fn children(&self) -> &[XmlElement] {
        &self.children
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XmlSource {
    pub structure_definitions: Vec<StructureDefinitionXml>,
    pub value_sets: Vec<ValueSetXml>,
    pub code_systems: Vec<CodeSystemXml>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureDefinitionXml {
    pub id: String,
    pub url: String,
    pub status: PublicationStatus,
    pub kind: StructureDefinitionKind,
    pub abstract_: bool,
    pub type_: String,
    pub base_definition: Option<String>,
    pub derivation: Option<Derivation>,
    elements: Vec<ElementDefinitionXml>,
}

impl StructureDefinitionXml {
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            status: PublicationStatus::Active,
            kind: StructureDefinitionKind::Resource,
            abstract_: false,
            type_: String::new(),
            base_definition: None,
            derivation: None,
            elements: Vec::new(),
        }
    }

    pub fn elements(self, elements: Vec<ElementDefinitionXml>) -> Self {
        let mut sd = self;
        sd.elements = elements;
        sd
    }

    pub fn get_elements(&self) -> &[ElementDefinitionXml] {
        &self.elements
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PublicationStatus {
    #[default]
    Draft,
    Active,
    Retired,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StructureDefinitionKind {
    #[default]
    Resource,
    Datatype,
    Primitive,
    ComplexType,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Derivation {
    Specialization,
    Constraint,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ElementDefinitionXml {
    pub path: String,
    pub short: Option<String>,
    pub definition: Option<String>,
    pub comment: Option<String>,
    pub alias: Vec<String>,
    pub min: u32,
    pub max: String,
    pub base: Option<BaseDefinitionXml>,
    pub type_: Vec<ElementTypeDefinitionXml>,
    pub constraints: Vec<ConstraintXml>,
    pub binding: Option<BindingXml>,
    pub is_modifier: bool,
    pub is_summary: bool,
    pub must_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseDefinitionXml {
    pub path: String,
    pub min: u32,
    pub max: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementTypeDefinitionXml {
    pub code: String,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintXml {
    pub id: String,
    pub severity: ConstraintSeverityXml,
    pub key: String,
    pub human: String,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintSeverityXml {
    Error,
    Warning,
    Information,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingXml {
    pub strength: BindingStrengthXml,
    pub value_set: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingStrengthXml {
    Required,
    Extensible,
    Preferred,
    Example,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValueSetXml {
    pub id: String,
    pub url: String,
    pub status: PublicationStatus,
    pub description: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub compose: Option<ComposeXml>,
}

impl ValueSetXml {
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeXml {
    pub import: Vec<String>,
    pub includes: Vec<ConceptSetXml>,
    pub excludes: Vec<ConceptSetXml>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptSetXml {
    pub system: Option<String>,
    pub version: Option<String>,
    pub concept: Vec<ConceptXml>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptXml {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeSystemXml {
    pub id: String,
    pub url: String,
    pub status: PublicationStatus,
    pub description: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub concept: Vec<ConceptXml>,
}

impl CodeSystemXml {
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            ..Self::default()
        }
    }
}
