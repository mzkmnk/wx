use std::path::Path;

use git2::Repository;

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
}
