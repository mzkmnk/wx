use std::{default, path::Path};

use crate::models::WtxError;

#[derive(Default)]
pub struct WorktreeManager;

impl WorktreeManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_worktree(
        &self,
        bare_repo_path: &Path,
        target_path: &Path,
        branch: &str,
    ) -> Result<(), WtxError> {
        todo!()
    }

    pub fn remove_worktree(&self) {
        todo!()
    }

    pub fn list_worktrees(&self) {
        todo!()
    }

    pub fn fetch(&self) {
        todo!()
    }

    pub fn get_remote_branches(&self) {
        todo!()
    }

    pub fn branch_exists(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::test_helpers::*;

    #[test]
    fn test_worktree_manager_create_worktree() {
        let (dir, base_dir) = setup_test_dirs();

        let bare_repo_path = create_test_bare_repo(&base_dir, "frontend");

        let target_path = dir.path().join("feature-auth/frontend");

        let worktree_manager = WorktreeManager::default();

        worktree_manager
            .create_worktree(&bare_repo_path, &target_path, "main")
            .unwrap();
    }
}
