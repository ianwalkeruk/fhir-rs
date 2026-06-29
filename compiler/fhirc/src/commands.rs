use std::fs;

use crate::cli::{Command, IrLoader, IrPath, IrValidator, Output};
use crate::error::{Error, IoError, YamlError};
use fhir_ir::{IrVersion, Specification};

pub struct CliCommands {
    compiler_version: String,
    ir_version: IrVersion,
    supported_releases: Vec<String>,
}

impl CliCommands {
    pub fn new() -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            ir_version: IrVersion::CURRENT,
            supported_releases: vec!["R4".to_string(), "R5".to_string()],
        }
    }

    pub fn loader() -> impl IrLoader {
        IrLoaderImpl
    }

    pub fn validator() -> impl IrValidator {
        IrValidatorImpl
    }

    pub fn formatter() -> impl IrFormatter {
        IrFormatterImpl
    }
}

impl Default for CliCommands {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IrLoaderImpl;

impl IrLoader for IrLoaderImpl {
    fn load(&self, path: IrPath) -> Result<Specification, Error> {
        let path = path.path();
        if !path.exists() {
            return Err(Error::FileNotFound(crate::error::FilePath::new(
                path.display().to_string(),
            )));
        }
        let content = fs::read_to_string(path)
            .map_err(|e| Error::Io(IoError::new(format!("Failed to read file: {}", e))))?;
        serde_yaml::from_str(&content)
            .map_err(|e| Error::YamlParse(YamlError::new(format!("Failed to parse YAML: {}", e))))
    }
}

pub struct IrValidatorImpl;

impl IrValidator for IrValidatorImpl {
    fn validate(&self, _spec: &Specification) -> Result<(), Error> {
        Ok(())
    }
}

pub struct IrFormatterImpl;

impl IrFormatter for IrFormatterImpl {
    fn format(&self, spec: &Specification) -> Result<Output, Error> {
        let yaml = serde_yaml::to_string(spec)
            .map_err(|e| Error::Io(IoError::new(format!("Failed to serialize IR: {}", e))))?;
        Ok(Output::new(yaml))
    }
}

pub use crate::cli::IrFormatter;

impl Command for CliCommands {
    fn version(&self) -> Result<Output, Error> {
        let output = format!(
            "fhirc version: {}\nIR version: {}\nSupported releases: {}",
            self.compiler_version,
            self.ir_version,
            self.supported_releases.join(", ")
        );
        Ok(Output::new(output))
    }

    fn schema(&self) -> Result<Output, Error> {
        let output = format!("IR schema version: {}", self.ir_version);
        Ok(Output::new(output))
    }

    fn validate_ir(&self, file: IrPath) -> Result<(), Error> {
        let loader = Self::loader();
        let validator = Self::validator();

        let spec = loader.load(file)?;
        validator.validate(&spec)?;
        Ok(())
    }

    fn pretty(&self, file: IrPath) -> Result<Output, Error> {
        let loader = Self::loader();
        let formatter = Self::formatter();

        let spec = loader.load(file)?;
        formatter.format(&spec)
    }

    fn canonicalise(&self, file: IrPath) -> Result<Output, Error> {
        let loader = Self::loader();
        let formatter = Self::formatter();

        let spec = loader.load(file)?;
        formatter.format(&spec)
    }

    fn parse(&self) -> Result<(), Error> {
        println!("not yet implemented: parse");
        Ok(())
    }

    fn pass(&self, _name: &str) -> Result<(), Error> {
        println!("not yet implemented: pass");
        Ok(())
    }

    fn generate(&self) -> Result<(), Error> {
        println!("not yet implemented: generate");
        Ok(())
    }

    fn diff(&self) -> Result<(), Error> {
        println!("not yet implemented: diff");
        Ok(())
    }
}
