use serde::{Deserialize, Serialize};

/// Represents a registered Git repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    /// Repository name (derived from URL, e.g., "frontend" from "git@github.com:org/frontend.git")
    pub name: String,
    /// Remote URL (SSH or HTTPS format)
    pub remote: String,
    /// Local path to the bare repository (e.g., "~/.wx/frontend.git")
    pub local_path: String,
}

impl Repository {
    /// Create a new Repository instance
    pub fn new(name: String, remote: String, local_path: String) -> Self {
        Self {
            name,
            remote,
            local_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_new() {
        let repo = Repository::new(
            "frontend".to_string(),
            "git@github.com:org/frontend.git".to_string(),
            "/home/user/.wx/frontend.git".to_string(),
        );

        assert_eq!(repo.name, "frontend");
        assert_eq!(repo.remote, "git@github.com:org/frontend.git");
        assert_eq!(repo.local_path, "/home/user/.wx/frontend.git");
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository::new(
            "backend".to_string(),
            "https://github.com/org/backend.git".to_string(),
            "/home/user/.wx/backend.git".to_string(),
        );

        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: Repository = serde_json::from_str(&json).unwrap();

        assert_eq!(repo, deserialized);
    }

    #[test]
    fn test_repository_json_format() {
        let repo = Repository::new(
            "test-repo".to_string(),
            "git@github.com:org/test-repo.git".to_string(),
            "/home/user/.wx/test-repo.git".to_string(),
        );

        let json = serde_json::to_string_pretty(&repo).unwrap();
        assert!(json.contains("\"name\": \"test-repo\""));
        assert!(json.contains("\"remote\": \"git@github.com:org/test-repo.git\""));
        assert!(json.contains("\"local_path\": \"/home/user/.wx/test-repo.git\""));
    }
}
