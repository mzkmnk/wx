use std::path::PathBuf;

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
    pub is_default: String,
}
