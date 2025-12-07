use clap::Parser;

use crate::cli::{Cli, Commands};

// Module declarations
pub mod cli;
pub mod commands;
pub mod config;
pub mod git;
pub mod models;
pub mod service;
pub mod tui;
pub mod utils;
pub mod workspace;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Register { url } => {}
        Commands::List => {}
        Commands::Unregister { name } => {
            todo!()
        }
    }
    Ok(())
}
