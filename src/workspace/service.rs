use crate::{
    config::manager::ConfigManager,
    models::WtxError,
    utils::get_wtx_home,
    workspace::{file::WorkspaceFileManager, worktree::WorktreeManager},
};

/// en: Service for generating workspaces with worktrees
///
/// ja: worktreeを含むworkspaceを生成するサービス
pub struct WorkspaceGenerationService<W: WorktreeManager> {
    config_manager: ConfigManager,
    worktree_manager: W,
    workspace_file_manager: WorkspaceFileManager,
}

impl<W: WorktreeManager> WorkspaceGenerationService<W> {
    pub fn new(worktree_manager: W) -> Result<Self, WtxError> {
        Ok(Self {
            config_manager: ConfigManager::new()?,
            workspace_file_manager: WorkspaceFileManager::default(),
            worktree_manager,
        })
    }

    /// en: Returns the list of branches for the specified repository
    ///
    /// ja: 指定したリポジトリのブランチを返却する
    pub fn get_branches(&self, repo_name: &str) -> Result<Vec<String>, WtxError> {
        let bare_repo_path = get_wtx_home().unwrap().join(format!("{}.git", repo_name));
        self.worktree_manager.get_remote_branches(&bare_repo_path)
    }
}

#[cfg(test)]
mod tests {

    use crate::workspace::worktree::MockWorktreeManager;

    use super::*;

    #[test]
    fn test_new() {
        let mock_worktree_manager = MockWorktreeManager::new();

        let workspace_generation_service = WorkspaceGenerationService::new(mock_worktree_manager);
        assert!(workspace_generation_service.is_ok());
    }

    #[test]
    fn test_get_ranches() {
        let mut mock_worktree_manager = MockWorktreeManager::new();

        mock_worktree_manager
            .expect_get_remote_branches()
            .returning(|_| Ok(vec!["origin/main".to_string(), "origin/dev".to_string()]));

        let workspace_generation_service =
            WorkspaceGenerationService::new(mock_worktree_manager).unwrap();

        let branches = workspace_generation_service.get_branches("wtx").unwrap();

        assert_eq!(
            branches,
            vec!["origin/main".to_string(), "origin/dev".to_string()]
        );
    }
}
