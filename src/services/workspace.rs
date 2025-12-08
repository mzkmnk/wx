use std::path::{Path, PathBuf};

use crate::{
    infrastructure::{filesystem::WorkspaceFileManager, git::WorktreeManager},
    models::{workspace::WorktreeSelection, WtxError},
};

/// en: Service for generating workspaces with worktrees
///
/// ja: worktreeを含むworkspaceを生成するサービス
pub struct WorkspaceGenerationService<W: WorktreeManager> {
    worktree_manager: W,
    workspace_file_manager: WorkspaceFileManager,
    wtx_home: PathBuf,
}

impl<W: WorktreeManager> WorkspaceGenerationService<W> {
    pub fn new(worktree_manager: W, wtx_home: PathBuf) -> Result<Self, WtxError> {
        Ok(Self {
            workspace_file_manager: WorkspaceFileManager,
            worktree_manager,
            wtx_home,
        })
    }

    /// en: Returns the list of branches for the specified repository
    ///
    /// ja: 指定したリポジトリのブランチを返却する
    pub fn get_branches(&self, repo_name: &str) -> Result<Vec<String>, WtxError> {
        let bare_repo_path = self.wtx_home.join(format!("{repo_name}.git"));
        self.worktree_manager.fetch(&bare_repo_path)?;
        self.worktree_manager.get_remote_branches(&bare_repo_path)
    }

    /// en: Generate workspace with worktrees from the specified selections
    ///
    /// ja: 指定された選択からworktreeを含むworkspaceを生成する
    pub fn generate(
        &self,
        workspace_dir: &Path,
        worktree_selection: Vec<WorktreeSelection>,
        workspace_name: &str,
    ) -> Result<(), WtxError> {
        // create workspace file
        self.workspace_file_manager.generate(
            workspace_dir,
            workspace_name,
            worktree_selection
                .iter()
                .map(|ws| ws.branch.clone())
                .collect(),
        )?;

        // create worktrees
        for ws in worktree_selection {
            let bare_repo_path = self.wtx_home.join(format!("{}.git", ws.repo_name));
            let target_path = workspace_dir.join(&ws.branch);
            self.worktree_manager
                .create_worktree(&bare_repo_path, &target_path, &ws.branch)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use git2::Repository;

    use crate::{
        infrastructure::git::{DefaultWorktreeManager, MockWorktreeManager},
        utils::test_helpers::{add_test_remote_branch, create_test_bare_repo, setup_test_dirs},
    };

    use super::*;

    #[test]
    fn test_new() {
        let (_dir, base_dir) = setup_test_dirs();
        let mock_worktree_manager = MockWorktreeManager::new();

        let workspace_generation_service =
            WorkspaceGenerationService::new(mock_worktree_manager, base_dir);
        assert!(workspace_generation_service.is_ok());
    }

    #[test]
    fn test_get_branches() {
        let (_dir, base_dir) = setup_test_dirs();
        let mut mock_worktree_manager = MockWorktreeManager::new();

        mock_worktree_manager.expect_fetch().returning(|_| Ok(()));
        mock_worktree_manager
            .expect_get_remote_branches()
            .returning(|_| Ok(vec!["origin/main".to_string(), "origin/dev".to_string()]));

        let workspace_generation_service =
            WorkspaceGenerationService::new(mock_worktree_manager, base_dir).unwrap();

        let branches = workspace_generation_service.get_branches("wtx").unwrap();

        assert_eq!(
            branches,
            vec!["origin/main".to_string(), "origin/dev".to_string()]
        );
    }

    #[test]
    fn test_generate() {
        let (dir, base_dir) = setup_test_dirs();
        let working_dir = dir.path().join("work/sso-feature");
        let wtx_frontend_repo_name = String::from("wtx-frontend");
        let wtx_backend_repo_name = String::from("wtx-backend");
        fs::create_dir_all(&working_dir).unwrap();
        let wtx_frontend_repo_path = create_test_bare_repo(&base_dir, &wtx_frontend_repo_name);
        let wtx_backend_repo_path = create_test_bare_repo(&base_dir, &wtx_backend_repo_name);
        let wtx_frontend_repo = Repository::open_bare(&wtx_frontend_repo_path).unwrap();
        let wtx_backend_repo = Repository::open_bare(&wtx_backend_repo_path).unwrap();
        add_test_remote_branch(&wtx_frontend_repo, "sso-ui");
        add_test_remote_branch(&wtx_backend_repo, "sso-api");
        let selection: Vec<WorktreeSelection> = vec![
            WorktreeSelection {
                repo_name: wtx_frontend_repo_name,
                branch: "sso-ui".to_string(),
            },
            WorktreeSelection {
                repo_name: wtx_backend_repo_name,
                branch: "sso-api".to_string(),
            },
        ];

        let worktree_manager = DefaultWorktreeManager;

        let workspace_generation_service =
            WorkspaceGenerationService::new(worktree_manager, base_dir).unwrap();

        assert!(workspace_generation_service
            .generate(&working_dir, selection, "wtx")
            .is_ok());

        assert!(working_dir.join("wtx.code-workspace").exists());
        assert!(working_dir.join("sso-ui").exists());
        assert!(working_dir.join("sso-api").exists());
    }
}
