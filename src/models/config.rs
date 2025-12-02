use serde::{Deserialize, Serialize};

use super::error::WtxError;
use super::repository::Repository;

/// Configuration file structure for wtx
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    /// List of registered repositories
    pub repositories: Vec<Repository>,
}

impl Config {
    /// Create a new empty Config
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    /// Add a repository to the configuration
    ///
    /// Returns an error if a repository with the same name is already registered
    pub fn add_repository(&mut self, repo: Repository) -> Result<(), WtxError> {
        if self.repositories.iter().any(|r| r.name == repo.name) {
            return Err(WtxError::AlreadyRegistered(repo.name));
        }
        self.repositories.push(repo);
        Ok(())
    }

    /// Remove a repository from the configuration by name
    ///
    /// Returns the removed repository, or an error if not found
    pub fn remove_repository(&mut self, name: &str) -> Result<Repository, WtxError> {
        let index = self
            .repositories
            .iter()
            .position(|r| r.name == name)
            .ok_or_else(|| WtxError::RepositoryNotFound(name.to_string()))?;
        Ok(self.repositories.remove(index))
    }

    /// Find a repository by name
    pub fn find_repository(&self, name: &str) -> Option<&Repository> {
        self.repositories.iter().find(|r| r.name == name)
    }

    /// Check if a repository with the given name exists
    pub fn has_repository(&self, name: &str) -> bool {
        self.repositories.iter().any(|r| r.name == name)
    }

    /// Check if a repository with the given remote URL exists
    pub fn has_remote(&self, remote: &str) -> bool {
        self.repositories.iter().any(|r| r.remote == remote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert!(config.repositories.is_empty());
    }

    #[test]
    fn test_config_add_repository() {
        let mut config = Config::new();
        assert!(config
            .add_repository(create_test_repository("frontend"))
            .is_ok());
        assert_eq!(config.repositories.len(), 1);
    }

    #[test]
    fn test_config_add_duplicate_repository() {
        let mut config = Config::new();
        assert!(config
            .add_repository(create_test_repository("frontend"))
            .is_ok());

        let result = config.add_repository(create_test_repository("frontend"));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WtxError::AlreadyRegistered(_)
        ));
    }

    #[test]
    fn test_config_remove_repository() {
        let mut config = Config::new();
        config
            .add_repository(create_test_repository("frontend"))
            .unwrap();

        let removed = config.remove_repository("frontend");
        assert!(removed.is_ok());
        assert!(config.repositories.is_empty());
    }

    #[test]
    fn test_config_remove_nonexistent_repository() {
        let mut config = Config::new();
        let result = config.remove_repository("nonexistent");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WtxError::RepositoryNotFound(_)
        ));
    }

    #[test]
    fn test_config_find_repository() {
        let mut config = Config::new();
        config
            .add_repository(create_test_repository("frontend"))
            .unwrap();

        let found = config.find_repository("frontend");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "frontend");

        let not_found = config.find_repository("backend");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_config_has_repository() {
        let mut config = Config::new();
        config
            .add_repository(create_test_repository("frontend"))
            .unwrap();

        assert!(config.has_repository("frontend"));
        assert!(!config.has_repository("backend"));
    }

    #[test]
    fn test_config_has_remote() {
        let mut config = Config::new();
        config
            .add_repository(create_test_repository("frontend"))
            .unwrap();

        assert!(config.has_remote("git@github.com:org/frontend.git"));
        assert!(!config.has_remote("git@github.com:org/backend.git"));
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::new();
        config
            .add_repository(create_test_repository("frontend"))
            .unwrap();
        config
            .add_repository(create_test_repository("backend"))
            .unwrap();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }
}
