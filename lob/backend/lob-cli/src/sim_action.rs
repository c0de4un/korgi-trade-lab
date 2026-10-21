//! Simulation-related CLI subcommands.

use clap::Subcommand;

/// Actions under `lob-cli sim`.
#[derive(Debug, Subcommand)]
pub enum SimAction {
    /// Execute a simulation run.
    Run {
        /// Dataset key or path.
        dataset: String,
    },
}
