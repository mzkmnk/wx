use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::models::{Config, Repository};

/// テスト用の一時ディレクトリとbase_dirをセットアップ
pub fn setup_test_dirs() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let base_dir = dir.path().join(".wtx");
    (dir, base_dir)
}

/// テスト用の通常Gitリポジトリを作成
pub fn create_test_git_repo(parent: &Path, name: &str) -> PathBuf {
    let repo_path = parent.join(name);
    git2::Repository::init(&repo_path).unwrap();
    repo_path
}

/// テスト用のbareリポジトリを作成
pub fn create_test_bare_repo(parent: &Path, name: &str) -> PathBuf {
    let repo_path = parent.join(format!("{}.git", name));
    git2::Repository::init_bare(&repo_path).unwrap();
    repo_path
}

/// テスト用のRepositoryモデルを生成
pub fn create_test_repository(name: &str) -> Repository {
    Repository::new(
        name.to_string(),
        format!("git@github.com:org/{}.git", name),
        format!("/home/user/.wtx/{}.git", name),
    )
}

/// テスト用のconfig.jsonを作成
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
/// en: Create a remote branch for testing
pub fn add_test_remote_branch(repo: &git2::Repository, branch_name: &str) {
    let tree_builder = repo.treebuilder(None).unwrap();
    let tree_oid = tree_builder.write().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    let sig = git2::Signature::now("m4i", "test@example.com").unwrap();

    let commit_oid = repo.commit(None, &sig, &sig, "Init", &tree, &[]).unwrap();

    repo.reference(
        &format!("refs/remotes/origin/{}", branch_name),
        commit_oid,
        true,
        "test setup",
    )
    .unwrap();
}
