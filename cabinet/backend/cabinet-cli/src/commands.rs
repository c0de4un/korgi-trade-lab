//! Top-level CLI commands for `cabinet-cli`.

use clap::Subcommand;

/// Subcommands exposed by the cabinet CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print package version.
    Version,
    /// Run database migrations (stub in M0).
    Migrate,
}
