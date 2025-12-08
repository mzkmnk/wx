use std::fs;

use dialoguer::{Input, MultiSelect};

use crate::{
    infrastructure::git::DefaultWorktreeManager,
    models::{workspace::WorktreeSelection, WtxError},
    services::{RepositoryService, WorkspaceGenerationService},
    utils::{get_current_dir, get_wtx_home},
};

/// en: Execute the `wtx new` command to create worktrees and a workspace file interactively
///
/// ja: `wtx new` コマンドを実行し、インタラクティブにworktreeとworkspaceファイルを作成する
pub fn execute(workspace_name: String) -> Result<(), WtxError> {
    let repos = RepositoryService::new()?.list()?;

    let repos_name: Vec<&str> = repos.iter().map(|repo| repo.name.as_str()).collect();

    let selected_repos = MultiSelect::new()
        .with_prompt("Select repositories")
        .items(&repos_name)
        .interact();

    let mut worktree_selection: Vec<WorktreeSelection> = Vec::new();

    match selected_repos {
        Ok(selected_repos) => {
            for idx in selected_repos {
                let branch_name: String = Input::new()
                    .with_prompt(format!("Branch for {}", repos[idx].name))
                    .interact_text()?;

                worktree_selection.push(WorktreeSelection {
                    repo_name: repos[idx].name.clone(),
                    branch: branch_name,
                });
            }

            let workspace_dir = get_current_dir()?.join(&workspace_name);
            fs::create_dir_all(&workspace_dir)?;

            match get_wtx_home() {
                Some(wtx_home) => {
                    let worktree_manager = DefaultWorktreeManager;

                    WorkspaceGenerationService::new(worktree_manager, wtx_home)?.generate(
                        &workspace_dir,
                        worktree_selection,
                        &workspace_name,
                    )?;
                }
                None => return Err(WtxError::HomeDirNotFound),
            }
        }
        Err(_) => {
            todo!()
        }
    }

    Ok(())
}
