use std::fs;

use dialoguer::{Input, MultiSelect};

use crate::{
    infrastructure::git::DefaultWorktreeManager,
    models::{workspace::WorktreeSelection, WxError},
    services::{RepositoryService, WorkspaceGenerationService},
    utils::{get_current_dir, get_wx_home},
};

/// en: Execute the `wx new` command to create worktrees and a workspace file interactively
///
/// ja: `wx new` コマンドを実行し、インタラクティブにworktreeとworkspaceファイルを作成する
pub fn execute(workspace_name: String) -> Result<(), WxError> {
    let repos = RepositoryService::new()?.list()?;
    if repos.is_empty() {
        return Err(WxError::General("No repositories registered. Please register a repository first using 'wx register <url>'".to_string()));
    }

    let repos_name: Vec<&str> = repos.iter().map(|repo| repo.name.as_str()).collect();

    let selected_repos = MultiSelect::new()
        .with_prompt("Select repositories")
        .items(&repos_name)
        .interact();

    let mut worktree_selection: Vec<WorktreeSelection> = Vec::new();

    match selected_repos {
        Ok(selected_repos) => {
            if selected_repos.is_empty() {
                return Err(WxError::General("No repositories selected".to_string()));
            }

            for idx in selected_repos {
                let branch_name: String = Input::new()
                    .with_prompt(format!("Branch for {name}", name = repos[idx].name))
                    .validate_with(|input: &String| -> Result<(), &str> {
                        let trimmed = input.trim();

                        if trimmed.is_empty() {
                            return Err("Branch name cannot be empty");
                        }

                        Ok(())
                    })
                    .interact_text()?;

                worktree_selection.push(WorktreeSelection {
                    repo_name: repos[idx].name.clone(),
                    branch: branch_name,
                });
            }

            let workspace_dir = get_current_dir()?.join(&workspace_name);
            if workspace_dir.exists() {
                return Err(WxError::General(format!(
                    "Workspace directory '{}' already exists",
                    workspace_name
                )));
            }
            fs::create_dir_all(&workspace_dir)?;

            let wx_home = get_wx_home().ok_or(WxError::HomeDirNotFound)?;
            let worktree_manager = DefaultWorktreeManager;

            WorkspaceGenerationService::new(worktree_manager, wx_home)?.generate(
                &workspace_dir,
                worktree_selection,
                &workspace_name,
            )?;
        }
        Err(_) => {
            return Err(WxError::General(
                "Repository selection was cancelled".to_string(),
            ));
        }
    }

    Ok(())
}
