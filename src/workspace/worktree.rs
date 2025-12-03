use std::path::Path;

use git2::{BranchType, Repository};

use crate::models::WtxError;

#[derive(Default)]
pub struct WorktreeManager;

impl WorktreeManager {
    pub fn fetch(&self, bare_repo_path: &Path) -> Result<(), WtxError> {
        let repo = Repository::open(bare_repo_path)?;

        let mut remote = repo.find_remote("origin")?;

        remote.fetch(&[] as &[&str], None, None)?;

        Ok(())
    }

    pub fn get_remote_branches(&self, bare_repo_path: &Path) -> Result<Vec<String>, WtxError> {
        let repo = Repository::open(bare_repo_path)?;
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

    pub fn branch_exists(
        &self,
        bare_repo_path: &Path,
        target_branch_name: &str,
    ) -> Result<bool, WtxError> {
        let repo = Repository::open(bare_repo_path)?;
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
}

#[cfg(test)]
mod tests {

    use crate::utils::test_helpers::{
        add_test_remote_branch, create_test_bare_repo, setup_test_dirs,
    };

    use super::*;

    #[test]
    fn test_worktree_manager_get_remote_branches() {
        let (_dir, base_dir) = setup_test_dirs();
        let bare_repo_path = create_test_bare_repo(&base_dir, "frontend");

        let bare_repo = Repository::open(&bare_repo_path).unwrap();

        add_test_remote_branch(&bare_repo, "main");
        add_test_remote_branch(&bare_repo, "auth");

        let worktree_manager = WorktreeManager::default();

        let remote_branches = worktree_manager
            .get_remote_branches(&bare_repo_path)
            .unwrap();

        assert_eq!(remote_branches.len(), 2);
    }

    #[test]
    fn test_worktree_manager_branch_exists() {
        let (_dir, base_dir) = setup_test_dirs();
        let bare_repo_path = create_test_bare_repo(&base_dir, "frontend");
        let bare_repo = Repository::open(&bare_repo_path).unwrap();
        add_test_remote_branch(&bare_repo, "main");
        add_test_remote_branch(&bare_repo, "auth");
        add_test_remote_branch(&bare_repo, "dashboard");

        let worktree_manager = WorktreeManager::default();

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
