use crate::cli::{Command, IrPath, Output};
use crate::error::{CommandName, Error};
use fhir_ir::IrVersion;

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
}

impl Default for CliCommands {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for CliCommands {
    fn version(&self) -> Result<Output, Error> {
        Err(Error::NotImplemented(CommandName::new("version")))
    }

    fn schema(&self) -> Result<Output, Error> {
        Err(Error::NotImplemented(CommandName::new("schema")))
    }

    fn validate_ir(&self, _file: IrPath) -> Result<(), Error> {
        Err(Error::NotImplemented(CommandName::new("validate-ir")))
    }

    fn pretty(&self, _file: IrPath) -> Result<Output, Error> {
        Err(Error::NotImplemented(CommandName::new("pretty")))
    }

    fn canonicalise(&self, _file: IrPath) -> Result<Output, Error> {
        Err(Error::NotImplemented(CommandName::new("canonicalise")))
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
