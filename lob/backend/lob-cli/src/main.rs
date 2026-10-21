//! LOB CLI: import, simulation, and benchmarks without full SaaS.

mod cli;
mod commands;
mod sim_action;

use clap::Parser;

use cli::Cli;
use commands::Commands;
use sim_action::SimAction;

fn main() -> anyhow::Result<()> {
    // Keep internal crates linked for the planned M1+ command surface.
    let _ = (
        std::any::type_name::<importers::ImporterError>(),
        std::any::type_name::<lob_core::LobCoreError>(),
        std::any::type_name::<sim_engine::SimEngineError>(),
        std::any::type_name::<storage::StorageError>(),
    );

    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("lob-cli {}", env!("CARGO_PKG_VERSION"));
            println!("Use --help for available commands.");
        }
        Some(Commands::Import { source }) => {
            println!("import stub: source={source} (not implemented in M0)");
        }
        Some(Commands::Sim {
            action: SimAction::Run { dataset },
        }) => {
            println!("sim run stub: dataset={dataset} (not implemented in M0)");
        }
        Some(Commands::Bench) => {
            println!("bench stub (not implemented in M0)");
        }
    }

    Ok(())
}
