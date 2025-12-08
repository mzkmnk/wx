use std::path::PathBuf;

use crate::models::WxError;

/// Get the wx home directory (~/.wx)
pub fn get_wx_home() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".wx"))
}

/// Get the config file path (~/.wx/config.json)
pub fn get_config_path() -> Option<PathBuf> {
    get_wx_home().map(|wx_home| wx_home.join("config.json"))
}

/// Get the backup config file path (~/.wx/config.backup.json)
pub fn get_backup_path() -> Option<PathBuf> {
    get_wx_home().map(|wx_home| wx_home.join("config.backup.json"))
}

/// en: Returns the current working directory path
///
/// ja: 現在いるディレクトリパスを返却する
pub fn get_current_dir() -> Result<PathBuf, WxError> {
    Ok(std::env::current_dir()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wx_home() {
        let wx_home = get_wx_home();
        assert!(wx_home.is_some());
        let path = wx_home.unwrap();
        assert!(path.ends_with(".wx"));
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
