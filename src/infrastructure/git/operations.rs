use std::path::Path;

use git2::build::RepoBuilder;
use regex::Regex;

use crate::models::WtxError;

#[derive(Default)]
pub struct GitOperations;

impl GitOperations {
    pub fn validate_url(&self, url: &str) -> Result<(), WtxError> {
        // allow local path
        if Path::new(url).exists() {
            return Ok(());
        }

        let https_pattern = Regex::new(r"^https://[\w\.\-]+/[\w\.\-_/]+?(?:\.git)?$").unwrap();
        let ssh_pattern = Regex::new(r"^git@[\w\.\-]+:[\w\.\-_/]+?(?:\.git)?$").unwrap();

        if https_pattern.is_match(url) || ssh_pattern.is_match(url) {
            Ok(())
        } else {
            Err(WtxError::InvalidUrl(url.to_string()))
        }
    }

    pub fn extract_repo_name(&self, url: &str) -> Result<String, WtxError> {
        // uses Path crate when local path
        if let Some(file_name) = Path::new(url).file_name() {
            let name = file_name.to_string_lossy();
            let repo_name = name.strip_suffix(".git").unwrap_or(&name).to_string();
            return Ok(repo_name);
        }

        let last_name = url.split("/").last().unwrap();
        let repo_name = last_name
            .strip_suffix(".git")
            .unwrap_or(last_name)
            .to_string();
        Ok(repo_name)
    }

    pub fn bare_clone(&self, url: &str, target_path: &Path) -> Result<(), WtxError> {
        RepoBuilder::new()
            .bare(true)
            .clone(url, target_path)
            .map(|_| ())
            .map_err(WtxError::GitError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::*;

    #[test]
    fn test_validate_url_https_valid() {
        let git_operations = GitOperations::default();

        assert!(git_operations
            .validate_url("https://github.com/org/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("https://github.com/org/repo")
            .is_ok());

        assert!(git_operations
            .validate_url("https://github.com/org/team/repo.git")
            .is_ok());
    }

    #[test]
    fn test_validate_url_ssh_valid() {
        let git_operations = GitOperations::default();

        assert!(git_operations
            .validate_url("git@github.com:org/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:org/repo")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:org/team/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:my_org/repo.git")
            .is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        let git_operations = GitOperations::default();

        assert!(git_operations.validate_url("").is_err());
        assert!(git_operations.validate_url("https://github.com").is_err());
        assert!(git_operations
            .validate_url("git@github.com/org/repo.git")
            .is_err());
    }

    #[test]
    fn test_extract_repo_name() {
        let git_operations = GitOperations::default();

        assert_eq!(
            git_operations
                .extract_repo_name("https://github.com/org/repo.git")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("https://github.com/org/repo")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:org/repo.git")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:my_org/repo.git")
                .unwrap(),
            "repo".to_string()
        );
    }

    #[test]
    fn test_bare_clone() {
        let (dir, _base_dir) = setup_test_dirs();

        let target_path = dir.path().join("target.git");
        let source_repo = create_test_git_repo(dir.path(), "source");

        let git_operations = GitOperations::default();

        assert!(git_operations
            .bare_clone(source_repo.to_str().unwrap(), &target_path)
            .is_ok());
        assert!(git2::Repository::open_bare(&target_path).is_ok());
    }
}
