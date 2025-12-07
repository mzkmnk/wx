use clap::Parser;
use color_eyre::eyre;
use console::style;

use crate::{
    cli::{Cli, Commands},
    models::WtxError,
};

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

fn main() -> color_eyre::Result<(), eyre::ErrReport> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Register { url } => match commands::register::execute(&url) {
            Ok(_) => println!("Registered: {}", style(url).cyan()),
            Err(e) => match e {
                WtxError::AlreadyRegistered(_) => {
                    println!("Already registered: {}", style(url).cyan())
                }
                _ => return Err(e.into()),
            },
        },
        Commands::List => {
            todo!()
        }
        Commands::Unregister { name } => {
            todo!()
        }
    }
    Ok(())
}
