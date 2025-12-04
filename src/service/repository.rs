use std::{fs::remove_dir_all, path::Path};

use crate::{
    config::manager::ConfigManager,
    git::operations::GitOperations,
    models::{Repository, WtxError},
};

pub struct RepositoryService {
    config_manager: ConfigManager,
    git_ops: GitOperations,
}

impl RepositoryService {
    pub fn new() -> Result<Self, WtxError> {
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

    pub fn register(&mut self, url: &str) -> Result<(), WtxError> {
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

    pub fn unregister(&mut self, repo_name: &str) -> Result<(), WtxError> {
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

    pub fn list(&self) -> Result<Vec<Repository>, WtxError> {
        Ok(self.config_manager.load()?.repositories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::*;

    #[test]
    fn test_repository_service_new() {
        let repository_service = RepositoryService::new();

        assert!(repository_service.is_ok());
    }

    #[test]
    fn test_repository_service_register_valid_url() {
        let (dir, base_dir) = setup_test_dirs();

        let soruce_repo = create_test_git_repo(dir.path(), "source_repo.git");
        let source_repo2 = create_test_git_repo(dir.path(), "source_repo2.git");

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
        let (dir, base_dir) = setup_test_dirs();
        let source_repo = create_test_git_repo(dir.path(), "source_repo.git");

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
        let (_dir, base_dir) = setup_test_dirs();
        let mut repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.register("https://github.com").is_err())
    }

    #[test]
    fn test_repository_service_list() {
        let (_dir, base_dir) = setup_test_dirs();

        create_test_config_file(&base_dir, vec![create_test_repository("test")]);

        let repository_service = RepositoryService::with_base_dir(&base_dir);

        assert!(repository_service.list().is_ok());
        assert_eq!(repository_service.list().unwrap().len(), 1);
    }

    #[test]
    fn test_repository_service_unregister() {
        let (_dir, base_dir) = setup_test_dirs();

        create_test_config_file(&base_dir, vec![create_test_repository("test")]);

        create_test_bare_repo(&base_dir, "test");

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
        let (_dir, base_dir) = setup_test_dirs();

        create_test_config_file(&base_dir, vec![create_test_repository("test")]);

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
