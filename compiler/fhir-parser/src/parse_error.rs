use std::fmt;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoSource(pub String);

impl fmt::Display for IoSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for IoSource {}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("IO error reading '{path}': {source}")]
    Io { path: PathBuf, source: IoSource },

    #[error("XML parse error at line {line}, column {column}: {message}")]
    XmlSyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Unexpected XML element: expected '{expected}', found '{actual}'")]
    UnexpectedElement { expected: String, actual: String },

    #[error("Missing required XML element: '{0}'")]
    MissingElement(String),

    #[error("Invalid attribute '{name}' on '{element}': {message}")]
    InvalidAttribute {
        element: String,
        name: String,
        message: String,
    },

    #[error("Invalid element content: {message}")]
    InvalidContent { message: String },

    #[error("FHIR StructureDefinition '{name}' is not well-formed: {message}")]
    InvalidStructureDefinition { name: String, message: String },

    #[error("FHIR ValueSet '{name}' is not well-formed: {message}")]
    InvalidValueSet { name: String, message: String },

    #[error("FHIR CodeSystem '{name}' is not well-formed: {message}")]
    InvalidCodeSystem { name: String, message: String },

    #[error("Multiple root elements found")]
    MultipleRootElements,

    #[error("Empty document")]
    EmptyDocument,

    #[error("Unsupported FHIR version: {0}")]
    UnsupportedVersion(String),
}

impl ParseError {
    pub fn io(path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        Self::Io {
            path: path.into(),
            source: IoSource(source.into()),
        }
    }

    pub fn xml_syntax(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::XmlSyntaxError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn unexpected_element(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::UnexpectedElement {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn missing_element(name: impl Into<String>) -> Self {
        Self::MissingElement(name.into())
    }

    pub fn invalid_attribute(
        element: impl Into<String>,
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidAttribute {
            element: element.into(),
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn invalid_content(message: impl Into<String>) -> Self {
        Self::InvalidContent {
            message: message.into(),
        }
    }

    pub fn invalid_structure_definition(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidStructureDefinition {
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn invalid_value_set(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidValueSet {
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn invalid_code_system(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidCodeSystem {
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn io_error(&self) -> Option<&PathBuf> {
        match self {
            ParseError::Io { path, .. } => Some(path),
            _ => None,
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
