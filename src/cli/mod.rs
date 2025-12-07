use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wtx")]
#[command(about = "Git worktree and workspace manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Register {
        url: String,
    }, // wtx register <url>
    List, // wtx list
    Unregister {
        name: String,
    }, // wtx unregister <name>
    New {
        #[arg(long)]
        workspace_name: Option<String>,
    }, // wtx new
}
