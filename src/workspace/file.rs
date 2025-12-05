use std::{fs, path::Path};

use crate::models::{
    workspace::{self, WorkspaceFile},
    WtxError,
};

#[derive(Default)]
pub struct WorkspaceFileManager;

impl WorkspaceFileManager {
    /// en: Generate a workspace file with the specified folders
    ///
    /// ja: 指定されたフォルダを含むworkspaceファイルを生成
    pub fn generate(
        &self,
        working_dir: &Path,
        workspace_name: &str,
        folders: Vec<String>,
    ) -> Result<(), WtxError> {
        let workspace_file = WorkspaceFile::new(folders);
        let workspace_file_path = working_dir.join(format!("{}.code-workspace", workspace_name));

        let workspace_file_json_string = serde_json::to_string_pretty(&workspace_file)?;
        fs::write(workspace_file_path, workspace_file_json_string)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{create_test_git_repo, setup_test_dirs};

    use super::*;

    #[test]
    fn test_generate() {
        let (dir, _base_dir) = setup_test_dirs();
        let parent_path = dir.path().join("work");
        let frontend_repo_path = create_test_git_repo(&parent_path, "frontend");
        let backend_repo_path = create_test_git_repo(&parent_path, "backend");

        let workspace_file_manager = WorkspaceFileManager::default();

        workspace_file_manager
            .generate(
                &parent_path,
                "wtx",
                vec![
                    frontend_repo_path.to_string_lossy().to_string(),
                    backend_repo_path.to_string_lossy().to_string(),
                ],
            )
            .unwrap();

        assert!(parent_path.join("wtx.code-workspace").exists());
    }
}
