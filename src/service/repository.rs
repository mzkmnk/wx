use std::{fs::remove_dir_all, path::Path};

use crate::{
    config::manager::ConfigManager,
    git::operations::GitOperations,
    models::{RegistrationError, Repository},
};

pub struct RepositoryService {
    config_manager: ConfigManager,
    git_ops: GitOperations,
}

impl RepositoryService {
    pub fn new() -> Result<Self, RegistrationError> {
        Ok(Self {
            config_manager: ConfigManager::new()?,
            git_ops: GitOperations,
        })
    }

    pub fn with_base_dir(base_dir: &Path) -> Self {
        Self {
            config_manager: ConfigManager::with_base_dir(base_dir),
            git_ops: GitOperations,
        }
    }

    pub fn register(&mut self, url: &str) -> Result<(), RegistrationError> {
        self.git_ops.validate_url(url)?;

        let repo_name = self.git_ops.extract_repo_name(url)?;

        let target_path = self
            .config_manager
            .base_dir()
            .join(format!("{}.git", &repo_name));

        self.git_ops.bare_clone(url, &target_path)?;

        let mut config = self.config_manager.load()?;

        config.add_repository(Repository {
            name: repo_name,
            remote: url.to_string(),
            local_path: target_path.to_str().unwrap().to_string(),
        })?;

        self.config_manager.save(&config)?;

        Ok(())
    }

    pub fn unregister(&mut self, repo_name: &str) -> Result<(), RegistrationError> {
        let mut config = self.config_manager.load()?;

        config.remove_repository(repo_name)?;

        self.config_manager.save(&config)?;

        remove_dir_all(
            self.config_manager
                .base_dir()
                .join(format!("{}.git", repo_name)),
        )?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Repository>, RegistrationError> {
        Ok(self.config_manager.load()?.repositories)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir_all, write};

    use tempfile::tempdir;

    use super::*;

    fn create_config_file(dir_path: &Path) {
        create_dir_all(dir_path).unwrap();
        let json = r#"{"repositories":[{"name":"test","remote":"git@github.com:org/test.git","local_path":"/home/user/.wtx/test.git"}]}"#;
        write(dir_path.join("config.json"), json).unwrap();
    }

    #[test]
    fn test_repository_service_new() {
        let repository_service = RepositoryService::new();

        assert!(repository_service.is_ok());
    }

    #[test]
    fn test_repository_service_register_valid_url() {
        let dir = tempdir().unwrap();

        let base_dir = dir.path().join(".wtx");

        let soruce_repo = dir.path().join("source_repo.git");
        git2::Repository::init(&soruce_repo).unwrap();
        let source_repo2 = dir.path().join("source_repo2.git");
        git2::Repository::init(&source_repo2).unwrap();

        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service
            .register(soruce_repo.to_str().unwrap())
            .is_ok());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            1
        );

        assert!(base_dir.join("source_repo.git").exists());

        assert!(repository_service
            .register(source_repo2.to_str().unwrap())
            .is_ok());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            2
        );

        assert!(base_dir.join("source_repo2.git").exists());
    }

    #[test]
    fn test_repository_service_register_is_not_same_url() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().join(".wtx");
        let source_repo = dir.path().join("source_repo.git");
        git2::Repository::init(&source_repo).unwrap();

        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service
            .register(source_repo.to_str().unwrap())
            .is_ok());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            1
        );

        assert!(repository_service
            .register(source_repo.to_str().unwrap())
            .is_err());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            1
        );
    }

    #[test]
    fn test_repository_service_register_invalid_url() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().join(".wtx");
        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.register("https://github.com").is_err())
    }

    #[test]
    fn test_repository_service_list() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().join(".wtx");

        create_config_file(&base_dir);

        let repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.list().is_ok());
        assert_eq!(repository_service.list().unwrap().len(), 1);
    }

    #[test]
    fn test_repository_service_unregister() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().join(".wtx");

        create_config_file(&base_dir);

        git2::Repository::init(base_dir.join("test.git")).unwrap();

        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.unregister("test").is_ok());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            0
        );

        assert!(!base_dir.join("test.git").exists());
    }

    #[test]
    fn test_repository_service_unregister_not_found() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().join(".wtx");

        create_config_file(&base_dir);

        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.unregister("test2").is_err());

        assert_eq!(
            repository_service
                .config_manager
                .load()
                .unwrap()
                .repositories
                .len(),
            1
        );
    }
}
