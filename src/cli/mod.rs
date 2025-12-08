use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wx")]
#[command(about = "Git worktree and workspace manager")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// en: Register a Git repository (bare clone)
    ///
    /// ja: Gitリポジトリを登録する（bare clone）
    Register { url: String },
    /// en: List registered repositories
    ///
    /// ja: 登録済みリポジトリを一覧表示する
    List,
    /// en: Unregister a repository
    ///
    /// ja: リポジトリの登録を解除する
    Unregister { name: String },
    /// en: Create a new workspace
    ///
    /// ja: 新しいワークスペースを作成する
    New { workspace_name: String },
}
