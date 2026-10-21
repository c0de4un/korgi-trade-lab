//! Root CLI parser for `lob-cli`.

use clap::Parser;

use crate::commands::Commands;

/// Command-line interface for the LOB toolkit.
#[derive(Debug, Parser)]
#[command(name = "lob-cli", version, about = "Korgi Trade-Lab LOB CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
