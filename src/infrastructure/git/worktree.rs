use std::path::Path;

use git2::{BranchType, Repository, WorktreeAddOptions, WorktreePruneOptions};
use mockall::automock;

use crate::models::WxError;

#[automock]
pub trait WorktreeManager {
    fn fetch(&self, bare_repo_path: &Path) -> Result<(), WxError>;
    fn get_remote_branches(&self, bare_repo_path: &Path) -> Result<Vec<String>, WxError>;
    fn branch_exists(
        &self,
        bare_repo_path: &Path,
        target_branch_name: &str,
    ) -> Result<bool, WxError>;
    fn create_worktree(
        &self,
        bare_repo_path: &Path,
        target_path: &Path,
        branch: &str,
    ) -> Result<(), WxError>;
    fn list_worktrees(&self, bare_repo_path: &Path) -> Result<Vec<String>, WxError>;
    fn remove_worktree(&self, bare_repo_path: &Path, worktree_name: &str) -> Result<(), WxError>;
}

/// en: Manager for Git worktree operations
///
/// ja: Git worktree操作を管理するマネージャー
#[derive(Default)]
pub struct DefaultWorktreeManager;

impl WorktreeManager for DefaultWorktreeManager {
    /// en: Fetch latest changes from remote repository
    ///
    /// ja: リモートリポジトリから最新の変更をフェッチ
    fn fetch(&self, bare_repo_path: &Path) -> Result<(), WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;

        let mut remote = repo.find_remote("origin")?;

        remote.fetch(&[] as &[&str], None, None)?;

        Ok(())
    }

    /// en: Get all remote branches from the bare repository
    ///
    /// ja: bareリポジトリから全てのリモートブランチを取得
    fn get_remote_branches(&self, bare_repo_path: &Path) -> Result<Vec<String>, WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;
        let branches = repo.branches(Some(BranchType::Remote))?;

        let mut remote_branches: Vec<String> = Vec::new();

        for branch in branches {
            let (branch, _branch_type) = branch?;
            if let Some(branch_name) = branch.name()? {
                remote_branches.push(branch_name.to_string());
            }
        }

        Ok(remote_branches)
    }

    /// en: Check if a branch exists in the remote repository
    ///
    /// ja: リモートリポジトリにブランチが存在するか確認
    fn branch_exists(
        &self,
        bare_repo_path: &Path,
        target_branch_name: &str,
    ) -> Result<bool, WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;
        let branches = repo.branches(Some(BranchType::Remote))?;

        for branch in branches {
            let (branch, _branch_type) = branch?;
            if let Some(branch_name) = branch.name()? {
                if target_branch_name == branch_name.strip_prefix("origin/").unwrap_or(branch_name)
                {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// en: Create a worktree from a bare repository for the specified branch
    ///
    /// ja: bareリポジトリから指定ブランチのworktreeを作成
    fn create_worktree(
        &self,
        bare_repo_path: &Path,
        target_path: &Path,
        branch: &str,
    ) -> Result<(), WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;

        // Try to find local branch first
        let branch = match repo.find_branch(branch, BranchType::Local) {
            Ok(b) => b,
            Err(_) => {
                // Try to find remote branch
                match repo.find_branch(&format!("origin/{branch}"), BranchType::Remote) {
                    Ok(remote) => {
                        // Create local branch from remote
                        let commit = remote.get().peel_to_commit()?;
                        repo.branch(branch, &commit, false)?
                    }
                    Err(_) => {
                        // Branch doesn't exist remotely, create new branch from HEAD
                        let head = repo.head()?;
                        let commit = head.peel_to_commit()?;
                        repo.branch(branch, &commit, false)?
                    }
                }
            }
        };

        let reference = branch.into_reference();

        let mut opts = WorktreeAddOptions::new();
        opts.reference(Some(&reference));

        let worktree_name = target_path.file_name().and_then(|n| n.to_str());

        match worktree_name {
            Some(name) => {
                repo.worktree(name, target_path, Some(&opts))?;
                Ok(())
            }
            None => Err(WxError::InvalidPath(
                target_path.to_string_lossy().to_string(),
            )),
        }
    }

    /// en: List all worktrees associated with the bare repository
    ///
    /// ja: bareリポジトリに関連付けられた全てのworktreeを一覧表示
    fn list_worktrees(&self, bare_repo_path: &Path) -> Result<Vec<String>, WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;
        let worktrees = repo.worktrees()?;

        Ok(worktrees.iter().flatten().map(String::from).collect())
    }

    /// en: Remove a worktree and prune its references from the bare repository
    ///
    /// ja: worktreeを削除し、bareリポジトリからその参照をprune
    fn remove_worktree(&self, bare_repo_path: &Path, worktree_name: &str) -> Result<(), WxError> {
        let repo = Repository::open_bare(bare_repo_path)?;
        let worktree = repo.find_worktree(worktree_name)?;

        let mut opts = WorktreePruneOptions::new();
        opts.valid(true).working_tree(true);

        worktree.prune(Some(&mut opts))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{
        add_test_remote_branch, create_test_bare_repo, setup_test_dirs,
    };
    use git2::Repository;

    use super::*;

    #[test]
    fn test_worktree_manager_get_remote_branches() {
        let (_dir, base_dir) = setup_test_dirs();
        let bare_repo_path = create_test_bare_repo(&base_dir, "frontend");

        let bare_repo = Repository::open_bare(&bare_repo_path).unwrap();

        add_test_remote_branch(&bare_repo, "main");
        add_test_remote_branch(&bare_repo, "auth");

        let worktree_manager = DefaultWorktreeManager;

        let remote_branches = worktree_manager
            .get_remote_branches(&bare_repo_path)
            .unwrap();

        assert_eq!(remote_branches.len(), 2);
    }

    #[test]
    fn test_worktree_manager_branch_exists() {
        let (_dir, base_dir) = setup_test_dirs();
        let bare_repo_path = create_test_bare_repo(&base_dir, "frontend");
        let bare_repo = Repository::open_bare(&bare_repo_path).unwrap();
        add_test_remote_branch(&bare_repo, "main");
        add_test_remote_branch(&bare_repo, "auth");
        add_test_remote_branch(&bare_repo, "dashboard");

        let worktree_manager = DefaultWorktreeManager;

        assert!(worktree_manager
            .branch_exists(&bare_repo_path, "main")
            .unwrap());

        assert!(worktree_manager
            .branch_exists(&bare_repo_path, "auth")
            .unwrap());

        assert!(worktree_manager
            .branch_exists(&bare_repo_path, "dashboard")
            .unwrap());

        assert!(!worktree_manager
            .branch_exists(&bare_repo_path, "feature")
            .unwrap());
    }
}
