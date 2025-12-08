use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wx")]
#[command(about = "Git worktree and workspace manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Register { url: String },       // wx register <url>
    List,                           // wx list
    Unregister { name: String },    // wx unregister <name>
    New { workspace_name: String }, // wx new <workspace_name>
}
