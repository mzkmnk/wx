use std::path::PathBuf;

/// Get the wtx home directory (~/.wtx)
pub fn get_wtx_home() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".wtx"))
}

/// Get the config file path (~/.wtx/config.json)
pub fn get_config_path() -> Option<PathBuf> {
    get_wtx_home().map(|wtx_home| wtx_home.join("config.json"))
}

/// Get the backup config file path (~/.wtx/config.backup.json)
pub fn get_backup_path() -> Option<PathBuf> {
    get_wtx_home().map(|wtx_home| wtx_home.join("config.backup.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wtx_home() {
        let wtx_home = get_wtx_home();
        assert!(wtx_home.is_some());
        let path = wtx_home.unwrap();
        assert!(path.ends_with(".wtx"));
    }

    #[test]
    fn test_get_config_path() {
        let config_path = get_config_path();
        assert!(config_path.is_some());
        let path = config_path.unwrap();
        assert!(path.ends_with("config.json"));
    }

    #[test]
    fn test_get_backup_path() {
        let backup_path = get_backup_path();
        assert!(backup_path.is_some());
        let path = backup_path.unwrap();
        assert!(path.ends_with("config.backup.json"));
    }
}
