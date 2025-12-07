use clap::Parser;
use console::style;

use crate::{
    cli::{Cli, Commands},
    models::WtxError,
};

// Module declarations
pub mod cli;
pub mod commands;
pub mod infrastructure;
pub mod models;
pub mod services;
pub mod tui;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Register { url } => match commands::register::execute(&url) {
            Ok(_) => println!("Registered: {}", style(url).cyan()),
            Err(e) => match e {
                WtxError::AlreadyRegistered(_) => {
                    println!(
                        "{} {}",
                        style("Already registered:").yellow(),
                        style(url).cyan()
                    )
                }
                _ => return Err(e.into()),
            },
        },
        Commands::List => {
            todo!()
        }
        Commands::Unregister { name: _name } => {
            todo!()
        }
    }
    Ok(())
}
