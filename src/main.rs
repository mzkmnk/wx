use clap::Parser;
use console::style;
use ptree::{print_config::StyleWhen, print_tree_with, Color, PrintConfig, Style, TreeBuilder};

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
        Commands::List => match commands::list::execute() {
            Ok(repos) => {
                if repos.is_empty() {
                    println!("{}", style("No registered repositories.").yellow())
                } else {
                    let config = PrintConfig {
                        styled: StyleWhen::Always,
                        leaf: Style {
                            foreground: Some(Color::Cyan),
                            ..Style::default()
                        },
                        ..PrintConfig::default()
                    };
                    for repo in &repos {
                        let tree = TreeBuilder::new(repo.name.clone())
                            .add_empty_child(repo.remote.clone())
                            .build();
                        print_tree_with(&tree, &config)?;
                    }
                }
            }
            Err(e) => return Err(e.into()),
        },
        Commands::New { workspace_name } => match commands::new::execute(workspace_name) {
            Ok(_) => {
                println!("{}", style("Workspace created.").green());
            }
            Err(e) => match e {
                WtxError::General(e) => {
                    println!("{}", style(e).red())
                }
                _ => return Err(e.into()),
            },
        },
        Commands::Unregister { name: _name } => {
            todo!()
        }
    }
    Ok(())
}
