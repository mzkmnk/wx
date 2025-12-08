use std::fs::{self, create_dir_all, write};
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::models::workspace::WorkspaceFile;
use crate::models::{Config, Repository};

/// en: Set up a temporary directory and base_dir for testing
///
/// ja: テスト用の一時ディレクトリとbase_dirをセットアップ
pub fn setup_test_dirs() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let base_dir = dir.path().join(".wx");
    (dir, base_dir)
}

/// en: Create a normal Git repository for testing
///
/// ja: テスト用の通常Gitリポジトリを作成
pub fn create_test_git_repo(parent: &Path, name: &str) -> PathBuf {
    let repo_path = parent.join(name);
    git2::Repository::init(&repo_path).unwrap();
    repo_path
}

/// en: Create a bare repository for testing
///
/// ja: テスト用のbareリポジトリを作成
pub fn create_test_bare_repo(parent: &Path, name: &str) -> PathBuf {
    let repo_path = parent.join(format!("{name}.git"));
    git2::Repository::init_bare(&repo_path).unwrap();
    repo_path
}

/// en: Generate a Repository model for testing
///
/// ja: テスト用のRepositoryモデルを生成
pub fn create_test_repository(name: &str) -> Repository {
    Repository::new(
        name.to_string(),
        format!("git@github.com:org/{name}.git"),
        format!("/home/user/.wx/{name}.git"),
    )
}

/// en: Create a config.json for testing
///
/// ja: テスト用のconfig.jsonを作成
pub fn create_test_config_file(base_dir: &Path, repos: Vec<Repository>) {
    create_dir_all(base_dir).unwrap();
    let mut config = Config::new();
    for repo in repos {
        config.add_repository(repo).unwrap();
    }
    let json = serde_json::to_string(&config).unwrap();
    write(base_dir.join("config.json"), json).unwrap();
}

/// ja: テスト用のremote branchの作成
///
/// en: Create a remote branch for testing
pub fn add_test_remote_branch(repo: &git2::Repository, branch_name: &str) {
    let tree_builder = repo.treebuilder(None).unwrap();
    let tree_oid = tree_builder.write().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    let sig = git2::Signature::now("m4i", "test@example.com").unwrap();

    let commit_oid = repo.commit(None, &sig, &sig, "Init", &tree, &[]).unwrap();

    repo.reference(
        &format!("refs/remotes/origin/{branch_name}"),
        commit_oid,
        true,
        "test setup",
    )
    .unwrap();
}

/// en: Create a workspace file for testing
///
/// ja: テスト用のworkspaceファイルを作成
pub fn test_create_workspace_file(working_dir: &Path, workspace_name: &str, folders: Vec<String>) {
    let workspace_file = WorkspaceFile::new(folders);
    let path = working_dir.join(format!("{workspace_name}.code-workspace"));
    let json = serde_json::to_string(&workspace_file).unwrap();
    fs::write(path, json).unwrap();
}
