use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wtx")]
#[command(about = "Git worktree and workspace manager")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Register { url: String },    // wtx register <url>
    List,                        // wtx list
    Unregister { name: String }, //wtx unregister <name>
}
