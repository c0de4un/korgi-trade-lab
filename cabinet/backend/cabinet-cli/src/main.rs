//! Cabinet CLI: admin and developer utilities.

mod cli;
mod commands;

use clap::Parser;

use cli::Cli;
use commands::Commands;

fn main() -> anyhow::Result<()> {
    let _ = std::any::type_name::<storage::StorageError>();

    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::Version) => {
            println!("cabinet-cli {}", env!("CARGO_PKG_VERSION"));
        }
        Some(Commands::Migrate) => {
            println!("migrate stub (not implemented in M0)");
        }
    }

    Ok(())
}
