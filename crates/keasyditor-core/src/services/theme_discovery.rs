/// Service for discovering installed Kvantum themes and Klassy presets.
use std::path::Path;

use crate::constants;
use crate::ini::parse_ini;
use crate::services::file_service::FileService;

/// Metadata about a discovered Kvantum theme.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeInfo {
    /// Display name of the theme (the directory name).
    pub name: String,
    /// Absolute path to the theme directory.
    pub path: String,
    /// Whether this is a system-installed theme.
    pub is_system: bool,
}

impl std::fmt::Display for ThemeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThemeInfo(\"{}\", system={})", self.name, self.is_system)
    }
}

pub struct ThemeDiscoveryService {
    file_service: FileService,
}

impl ThemeDiscoveryService {
    pub fn new(file_service: FileService) -> Self {
        Self { file_service }
    }

    /// Discover all installed Kvantum themes.
    ///
    /// Scans both the user directory and the system directory.
    /// User themes appear first in the returned list.
    pub fn discover_kvantum_themes(&self) -> Vec<ThemeInfo> {
        let mut themes = Vec::new();

        // User themes
        let user_dir = constants::kvantum_config_dir()
            .to_string_lossy()
            .into_owned();
        themes.extend(self.scan_directory(&user_dir, false));

        // System themes
        let system_dir = constants::kvantum_system_dir()
            .to_string_lossy()
            .into_owned();
        themes.extend(self.scan_directory(&system_dir, true));

        themes
    }

    fn scan_directory(&self, dir_path: &str, is_system: bool) -> Vec<ThemeInfo> {
        if !self.file_service.directory_exists(dir_path) {
            return Vec::new();
        }

        let entries = match self.file_service.list_directory(dir_path) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

        let mut themes = Vec::new();

        for entry_path in &entries {
            let name = Path::new(entry_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // Skip the global config file and hidden entries
            if name == "kvantum.kvconfig" || name.starts_with('.') {
                continue;
            }

            // Only consider directories
            if !self.file_service.directory_exists(entry_path) {
                continue;
            }

            // Verify a .kvconfig file exists
            let kvconfig_path = format!("{}/{}.kvconfig", entry_path, name);
            if !self.file_service.file_exists(&kvconfig_path) {
                continue;
            }

            themes.push(ThemeInfo {
                name,
                path: entry_path.clone(),
                is_system,
            });
        }

        themes.sort_by(|a, b| a.name.cmp(&b.name));
        themes
    }

    /// Discover the names of all Klassy presets.
    pub fn discover_klassy_presets(&self) -> Vec<String> {
        let presets_path = constants::klassy_presets_path()
            .to_string_lossy()
            .into_owned();
        if !self.file_service.file_exists(&presets_path) {
            return Vec::new();
        }

        let content = match self.file_service.read_file(&presets_path) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let doc = parse_ini(&content);
        doc.section_names().into_iter().map(|s| s.to_string()).collect()
    }
}
