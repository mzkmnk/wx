use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WorktreeSelection {
    pub repo_name: String,
    pub branch: String,
}

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub repo_name: String,
}

#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceSettings {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub folders: Vec<WorkspaceFolder>,
    pub settings: WorkspaceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub path: String,
}

impl WorkspaceFile {
    pub fn new(folders: Vec<String>) -> Self {
        Self {
            folders: folders
                .into_iter()
                .map(|f| WorkspaceFolder { path: f })
                .collect(),
            settings: WorkspaceSettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CleanTarget {
    All,              // wtx clean --all
    Worktree(String), // wtx clean <worktree>
}

#[derive(Debug)]
pub struct GenerationResult {
    pub worktrees: Vec<PathBuf>,
    pub workspace_file: PathBuf,
}

#[derive(Debug)]
pub struct CleanResult {
    pub removed_worktrees: Vec<PathBuf>,
    pub removed_workspace_files: Vec<PathBuf>,
    pub warnings: Vec<String>,
}
