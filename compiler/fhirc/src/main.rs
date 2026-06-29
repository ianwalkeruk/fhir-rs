mod cli;
mod commands;
mod error;
mod exit_code;

use clap::Parser;
use cli::{Cli, Command, Commands, IrPath, Output};

fn main() {
    let args = Cli::parse();
    let cli = commands::CliCommands::new();

    let result = match args.command {
        Commands::Version => cli.version().map(|o| println!("{}", o.into_string())),
        Commands::Schema => cli.schema().map(|o| println!("{}", o.into_string())),
        Commands::ValidateIr { file } => cli.validate_ir(IrPath::new(file)).map(|_| ()),
        Commands::Pretty { file } => cli
            .pretty(IrPath::new(file))
            .map(|o| println!("{}", o.into_string())),
        Commands::Canonicalise { file } => cli
            .canonicalise(IrPath::new(file))
            .map(|o| println!("{}", o.into_string())),
        Commands::Parse => cli.parse(),
        Commands::Pass { name } => cli.pass(&name),
        Commands::Generate => cli.generate(),
        Commands::Diff => cli.diff(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(exit_code::from_error(&e));
    }
}
