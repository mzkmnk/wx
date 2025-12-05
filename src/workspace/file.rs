use std::{fs, path::Path};

use crate::models::{workspace::WorkspaceFile, WtxError};

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

        if workspace_file_path.exists() {
            return Err(WtxError::WorkspaceFileAlreadyExists(
                workspace_file_path.to_string_lossy().to_string(),
            ));
        }

        let workspace_file_json_string = serde_json::to_string_pretty(&workspace_file)?;
        fs::write(workspace_file_path, workspace_file_json_string)?;

        Ok(())
    }

    /// en: Read a workspace file
    ///
    /// ja: workspaceファイルを読み込む
    pub fn read(&self, path: &Path) -> Result<WorkspaceFile, WtxError> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{
        create_test_git_repo, setup_test_dirs, test_create_workspace_file,
    };

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

    #[test]
    fn test_generate_duplicate_workspace() {
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

        assert!(workspace_file_manager
            .generate(
                &parent_path,
                "wtx",
                vec![
                    frontend_repo_path.to_string_lossy().to_string(),
                    backend_repo_path.to_string_lossy().to_string(),
                ],
            )
            .is_err());

        assert!(parent_path.join("wtx.code-workspace").exists());
    }

    #[test]
    fn test_read() {
        let (dir, _base_dir) = setup_test_dirs();
        let parent_path = dir.path().join("work");
        let frontend_repo_path = create_test_git_repo(&parent_path, "frontend");
        let backend_repo_path = create_test_git_repo(&parent_path, "backend");

        test_create_workspace_file(
            &parent_path,
            "wtx",
            vec![
                frontend_repo_path.to_string_lossy().to_string(),
                backend_repo_path.to_string_lossy().to_string(),
            ],
        );

        let workspace_file_manager = WorkspaceFileManager::default();

        let workspace_file = workspace_file_manager
            .read(&parent_path.join("wtx.code-workspace"))
            .unwrap();

        assert_eq!(workspace_file.folders.len(), 2);
        assert_eq!(
            workspace_file.folders[0].path,
            frontend_repo_path.to_string_lossy().to_string()
        );
        assert_eq!(
            workspace_file.folders[1].path,
            backend_repo_path.to_string_lossy().to_string()
        );
    }
}
