use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Failed to read file: {0}")]
    FileRead(String),

    #[error("Failed to parse {resource_type} '{name}': {message}")]
    Parse {
        resource_type: String,
        name: String,
        message: String,
    },

    #[error("Resource '{0}' not found")]
    NotFound(String),

    #[error("Invalid FHIR version: {0}")]
    InvalidVersion(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("XML error: {0}")]
    Xml(String),
}

pub type LoaderResult<T> = Result<T, LoaderError>;