use std::fs::{copy, create_dir_all, read_to_string, remove_file, write};
use std::path::{Path, PathBuf};

use serde_json::{from_str, to_string_pretty};

use crate::models::error::WtxError;
use crate::models::Config;

pub struct ConfigManager {
    pub config_path: PathBuf,
    pub backup_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, WtxError> {
        let base_dir = dirs::home_dir()
            .ok_or(WtxError::HomeDirNotFound)?
            .join(".wtx");

        Ok(Self::with_base_dir(&base_dir))
    }

    pub fn with_base_dir(base_dir: &Path) -> Self {
        Self {
            config_path: base_dir.join("config.json"),
            backup_path: base_dir.join("config.json.bak"),
        }
    }

    pub fn load(&self) -> Result<Config, WtxError> {
        if !self.config_path.exists() {
            return Ok(Config::new());
        }

        let content = read_to_string(&self.config_path)?;
        let config = from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<(), WtxError> {
        if let Some(parent_path) = self.config_path.parent() {
            create_dir_all(parent_path)?;
        }

        let content = to_string_pretty(config)?;

        write(&self.config_path, content)?;
        Ok(())
    }

    pub fn create_backup(&self) -> Result<(), WtxError> {
        if !self.config_path.exists() {
            return Err(WtxError::BackupError(
                "Cannot create backup: config.json not found".to_string(),
            ));
        }

        copy(&self.config_path, &self.backup_path)?;

        Ok(())
    }

    pub fn restore_backup(&self) -> Result<(), WtxError> {
        if !self.backup_path.exists() {
            return Err(WtxError::RestoreError(
                "Cannot restore backup: config.json.bak not found".to_string(),
            ));
        }

        copy(&self.backup_path, &self.config_path)?;

        Ok(())
    }

    pub fn delete_backup(&self) -> Result<(), WtxError> {
        if self.backup_path.exists() {
            remove_file(&self.backup_path)?;
        }

        Ok(())
    }

    pub fn base_dir(&self) -> &Path {
        self.config_path.parent().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{create_dir_all, write},
        vec,
    };

    use tempfile::{tempdir, TempDir};

    use crate::models::Repository;

    use super::*;

    fn create_config() -> Config {
        Config {
            repositories: vec![Repository::new(
                "frontend".to_string(),
                "git@github.com:org/test.git".to_string(),
                "/home/user/.wtx/test.git".to_string(),
            )],
        }
    }

    fn create_temp_dir() -> TempDir {
        tempdir().unwrap()
    }

    fn create_wtx_dir_path(dir: &TempDir) -> PathBuf {
        dir.path().join(".wtx")
    }

    fn create_config_file(dir_path: &PathBuf) {
        create_dir_all(dir_path).unwrap();
        let json = r#"{"repositories":[{"name":"test","remote":"git@github.com:org/test.git","local_path":"/home/user/.wtx/test.git"}]}"#;
        write(dir_path.join("config.json"), json).unwrap();
    }

    fn create_config_backup_file(dir_path: &PathBuf) {
        create_dir_all(dir_path).unwrap();
        let json = r#"{"repositories":[{"name":"test","remote":"git@github.com:org/test.git","local_path":"/home/user/.wtx/test.git"}]}"#;
        write(dir_path.join("config.json.bak"), json).unwrap();
    }

    #[test]
    fn test_config_manager_new() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);
        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);

        assert_eq!(config_manager.config_path, wtx_dir_path.join("config.json"));
        assert_eq!(
            config_manager.backup_path,
            wtx_dir_path.join("config.json.bak")
        );
    }

    #[test]
    fn test_config_manager_load() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);

        create_config_file(&wtx_dir_path);

        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);

        let config = config_manager.load().unwrap();
        assert_eq!(config.repositories.len(), 1);
    }

    #[test]
    fn test_config_manager_save() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);
        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);
        let config = create_config();

        assert!(config_manager.save(&config).is_ok());
        assert!(wtx_dir_path.join("config.json").exists());
    }

    #[test]
    fn test_config_manager_create_backup() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);

        create_config_file(&wtx_dir_path);

        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);

        assert!(config_manager.create_backup().is_ok());
        assert!(wtx_dir_path.join("config.json.bak").exists());
    }

    #[test]
    fn test_config_manager_restore_backup() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);

        create_config_backup_file(&wtx_dir_path);

        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);

        assert!(config_manager.restore_backup().is_ok());
        assert!(wtx_dir_path.join("config.json").exists());
    }

    #[test]
    fn test_config_manager_delete_backup() {
        let dir = create_temp_dir();
        let wtx_dir_path = create_wtx_dir_path(&dir);

        create_config_backup_file(&wtx_dir_path);

        let config_manager = ConfigManager::with_base_dir(&wtx_dir_path);

        assert!(config_manager.delete_backup().is_ok());
        assert!(!wtx_dir_path.join("config.json.bak").exists());
    }
}
