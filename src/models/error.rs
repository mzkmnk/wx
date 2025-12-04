use thiserror::Error;

/// Unified error type for wtx operations
#[derive(Error, Debug)]
pub enum WtxError {
    // Repository registration errors
    #[error("Invalid Git URL format: {0}. Expected SSH (git@host:path) or HTTPS (https://host/path) format")]
    InvalidUrl(String),

    #[error("Repository '{0}' is already registered")]
    AlreadyRegistered(String),

    #[error("Repository '{0}' not found")]
    RepositoryNotFound(String),

    // Worktree errors
    #[error("Invalid path: '{0}'")]
    InvalidPath(String),

    #[error("Worktree already exists at '{0}'")]
    WorktreeAlreadyExists(String),

    #[error("Branch '{0}' not found in repository '{1}'")]
    BranchNotFound(String, String),

    // Workspace errors
    #[error("Workspace file already exists: '{0}'")]
    WorkspaceFileAlreadyExists(String),

    // Common errors
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO operation failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Home directory not found")]
    HomeDirNotFound,

    #[error("Failed to create backup: {0}")]
    BackupError(String),

    #[error("Failed to restore backup: {0}")]
    RestoreError(String),

    #[error("Rollback failed after error: {original_error}, rollback error: {rollback_error}")]
    RollbackFailed {
        original_error: String,
        rollback_error: String,
    },
}

impl WtxError {
    /// Create a configuration error with a custom message
    pub fn config(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Create a backup error with a custom message
    pub fn backup(msg: impl Into<String>) -> Self {
        Self::BackupError(msg.into())
    }

    /// Create a restore error with a custom message
    pub fn restore(msg: impl Into<String>) -> Self {
        Self::RestoreError(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url_error_message() {
        let error = WtxError::InvalidUrl("bad-url".to_string());
        assert!(error.to_string().contains("Invalid Git URL format"));
        assert!(error.to_string().contains("bad-url"));
    }

    #[test]
    fn test_already_registered_error_message() {
        let error = WtxError::AlreadyRegistered("my-repo".to_string());
        assert!(error.to_string().contains("already registered"));
        assert!(error.to_string().contains("my-repo"));
    }

    #[test]
    fn test_repository_not_found_error_message() {
        let error = WtxError::RepositoryNotFound("missing-repo".to_string());
        assert!(error.to_string().contains("not found"));
        assert!(error.to_string().contains("missing-repo"));
    }

    #[test]
    fn test_config_error_helper() {
        let error = WtxError::config("custom config error");
        assert!(error.to_string().contains("custom config error"));
    }
}
