use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::error::Error;

#[derive(Debug, Clone, Parser)]
#[command(name = "fhirc")]
#[command(about = "FHIR compiler command-line interface")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(about = "Display version information")]
    Version,

    #[command(about = "Display IR schema version")]
    Schema,

    #[command(about = "Validate an IR document")]
    ValidateIr {
        #[arg(help = "Path to IR file")]
        file: PathBuf,
    },

    #[command(about = "Pretty-print an IR document")]
    Pretty {
        #[arg(help = "Path to IR file")]
        file: PathBuf,
    },

    #[command(about = "Normalise IR ordering and formatting")]
    Canonicalise {
        #[arg(help = "Path to IR file")]
        file: PathBuf,
    },

    #[command(about = "Parse external formats to IR (not yet implemented)")]
    Parse,

    #[command(about = "Run a compiler pass (not yet implemented)")]
    Pass {
        #[arg(help = "Pass name")]
        name: String,
    },

    #[command(about = "Generate output (not yet implemented)")]
    Generate,

    #[command(about = "Diff IR documents (not yet implemented)")]
    Diff,
}

/// Semantic type for IR file paths
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrPath(PathBuf);

impl IrPath {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }
}

impl From<PathBuf> for IrPath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

/// Semantic type for output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output(String);

impl Output {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

/// Trait for loading IR documents
pub trait IrLoader {
    fn load(&self, path: IrPath) -> Result<fhir_ir::Specification, Error>;
}

/// Trait for validating IR documents
pub trait IrValidator {
    fn validate(&self, spec: &fhir_ir::Specification) -> Result<(), Error>;
}

/// Trait for formatting IR output
pub trait IrFormatter {
    fn format(&self, spec: &fhir_ir::Specification) -> Result<Output, Error>;
}

pub trait Command {
    fn version(&self) -> Result<Output, Error>;

    fn schema(&self) -> Result<Output, Error>;

    fn validate_ir(&self, _file: IrPath) -> Result<(), Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "validate-ir",
        )))
    }

    fn pretty(&self, _file: IrPath) -> Result<Output, Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "pretty",
        )))
    }

    fn canonicalise(&self, _file: IrPath) -> Result<Output, Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "canonicalise",
        )))
    }

    fn parse(&self) -> Result<(), Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "parse",
        )))
    }

    fn pass(&self, _name: &str) -> Result<(), Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "pass",
        )))
    }

    fn generate(&self) -> Result<(), Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "generate",
        )))
    }

    fn diff(&self) -> Result<(), Error> {
        Err(Error::NotImplemented(crate::error::CommandName::new(
            "diff",
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_version_parses_no_args() {
        let cli = Cli::try_parse_from(["fhirc", "version"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(matches!(cli.command, Commands::Version));
    }

    #[test]
    fn test_cli_schema_parses_no_args() {
        let cli = Cli::try_parse_from(["fhirc", "schema"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(matches!(cli.command, Commands::Schema));
    }

    #[test]
    fn test_cli_validate_ir_parses_file_arg() {
        let cli = Cli::try_parse_from(["fhirc", "validate-ir", "test.ir.yaml"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::ValidateIr { file } => {
                assert_eq!(file, PathBuf::from("test.ir.yaml"));
            }
            _ => panic!("Expected ValidateIr command"),
        }
    }

    #[test]
    fn test_cli_pretty_parses_file_arg() {
        let cli = Cli::try_parse_from(["fhirc", "pretty", "test.ir.yaml"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::Pretty { file } => {
                assert_eq!(file, PathBuf::from("test.ir.yaml"));
            }
            _ => panic!("Expected Pretty command"),
        }
    }

    #[test]
    fn test_cli_canonicalise_parses_file_arg() {
        let cli = Cli::try_parse_from(["fhirc", "canonicalise", "test.ir.yaml"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::Canonicalise { file } => {
                assert_eq!(file, PathBuf::from("test.ir.yaml"));
            }
            _ => panic!("Expected Canonicalise command"),
        }
    }

    #[test]
    fn test_cli_parse_parses_no_args() {
        let cli = Cli::try_parse_from(["fhirc", "parse"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(matches!(cli.command, Commands::Parse));
    }

    #[test]
    fn test_cli_pass_parses_name_arg() {
        let cli = Cli::try_parse_from(["fhirc", "pass", "resolve-references"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::Pass { name } => {
                assert_eq!(name, "resolve-references");
            }
            _ => panic!("Expected Pass command"),
        }
    }

    #[test]
    fn test_cli_generate_parses_no_args() {
        let cli = Cli::try_parse_from(["fhirc", "generate"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(matches!(cli.command, Commands::Generate));
    }

    #[test]
    fn test_cli_diff_parses_no_args() {
        let cli = Cli::try_parse_from(["fhirc", "diff"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(matches!(cli.command, Commands::Diff));
    }

    #[test]
    fn test_ir_path_new() {
        let path = IrPath::new(PathBuf::from("test.ir.yaml"));
        assert_eq!(path.path(), &PathBuf::from("test.ir.yaml"));
    }

    #[test]
    fn test_output_new_and_into_string() {
        let output = Output::new("test output".to_string());
        assert_eq!(output.into_string(), "test output");
    }
}
