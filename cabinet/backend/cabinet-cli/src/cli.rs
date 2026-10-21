//! Root CLI parser for `cabinet-cli`.

use clap::Parser;

use crate::commands::Commands;

/// Command-line interface for cabinet admin utilities.
#[derive(Debug, Parser)]
#[command(name = "cabinet-cli", version, about = "Korgi Trade-Lab Cabinet CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
