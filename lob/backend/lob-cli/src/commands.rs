//! Top-level CLI commands for `lob-cli`.

use clap::Subcommand;

use crate::sim_action::SimAction;

/// Subcommands exposed by the LOB CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Import market data into the chunk store (stub in M0).
    Import {
        /// Path to the source file or descriptor.
        source: String,
    },
    /// Run a simulation (stub in M0).
    Sim {
        #[command(subcommand)]
        action: SimAction,
    },
    /// Run micro-benchmarks (stub in M0).
    Bench,
}
