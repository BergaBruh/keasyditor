use std::path::PathBuf;

/// Path to the Klassy configuration file.
pub fn klassy_config_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".config/klassy/klassyrc")
}

/// Path to the Klassy presets file.
pub fn klassy_presets_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".config/klassy/windecopresetsrc")
}

/// Path to the Kvantum user config directory.
pub fn kvantum_config_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".config/Kvantum")
}

/// Path to the Kvantum system themes directory.
pub fn kvantum_system_dir() -> PathBuf {
    PathBuf::from("/usr/share/Kvantum")
}

/// Path to the Kvantum global config file.
pub fn kvantum_global_config() -> PathBuf {
    kvantum_config_dir().join("kvantum.kvconfig")
}

/// Path to the KEasyDitor app settings file.
pub fn keasyditor_settings_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".config/keasyditor/settings.ini")
}

/// Path to the KEasyDitor cache directory.
pub fn keasyditor_cache_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".cache/keasyditor")
}

/// Path to the recent files list (non-essential cached data).
pub fn keasyditor_recent_files_path() -> PathBuf {
    keasyditor_cache_dir().join("recent_files")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn klassy_config_path_ends_correctly() {
        let p = klassy_config_path();
        assert!(p.ends_with(".config/klassy/klassyrc"));
    }

    #[test]
    fn klassy_presets_path_ends_correctly() {
        let p = klassy_presets_path();
        assert!(p.ends_with(".config/klassy/windecopresetsrc"));
    }

    #[test]
    fn kvantum_config_dir_ends_correctly() {
        let p = kvantum_config_dir();
        assert!(p.ends_with(".config/Kvantum"));
    }

    #[test]
    fn kvantum_system_dir_is_absolute() {
        let p = kvantum_system_dir();
        assert_eq!(p, PathBuf::from("/usr/share/Kvantum"));
    }

    #[test]
    fn kvantum_global_config_ends_correctly() {
        let p = kvantum_global_config();
        assert!(p.ends_with(".config/Kvantum/kvantum.kvconfig"));
    }

    #[test]
    fn keasyditor_settings_path_ends_correctly() {
        let p = keasyditor_settings_path();
        assert!(p.ends_with(".config/keasyditor/settings.ini"));
    }

    #[test]
    fn keasyditor_cache_dir_ends_correctly() {
        let p = keasyditor_cache_dir();
        assert!(p.ends_with(".cache/keasyditor"));
    }

    #[test]
    fn keasyditor_recent_files_path_ends_correctly() {
        let p = keasyditor_recent_files_path();
        assert!(p.ends_with(".cache/keasyditor/recent_files"));
    }

    #[test]
    fn paths_are_absolute() {
        assert!(klassy_config_path().is_absolute());
        assert!(kvantum_config_dir().is_absolute());
        assert!(keasyditor_settings_path().is_absolute());
        assert!(keasyditor_cache_dir().is_absolute());
        assert!(keasyditor_recent_files_path().is_absolute());
    }
}
