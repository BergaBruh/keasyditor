/// Service layer for Kvantum theme management.
///
/// Handles loading, saving, creating, and cloning Kvantum themes, as well as
/// reading and writing the global active-theme setting.
use std::io;
use std::path::Path;

use crate::constants;
use crate::ini::{parse_ini, serialize_ini, IniDocument};
use crate::models::kvantum::KvantumConfig;
use crate::services::file_service::FileService;

/// Container for a fully loaded Kvantum theme.
#[derive(Clone, Debug)]
pub struct KvantumThemeData {
    /// The parsed configuration from the theme's `.kvconfig` file.
    pub config: KvantumConfig,
    /// Raw SVG content (the theme's `.svg` file), or `None` if the theme has no SVG.
    pub svg_content: Option<String>,
    /// The display name of the theme (derived from the directory name).
    pub theme_name: String,
    /// Absolute path to the theme directory.
    pub dir_path: String,
}

impl std::fmt::Display for KvantumThemeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KvantumThemeData(\"{}\", hasSvg={})",
            self.theme_name,
            self.svg_content.is_some()
        )
    }
}

pub struct KvantumService {
    file_service: FileService,
}

impl KvantumService {
    pub fn new(file_service: FileService) -> Self {
        Self { file_service }
    }

    // --------------------------------------------------------------------------
    // Theme I/O
    // --------------------------------------------------------------------------

    /// Load a Kvantum theme from `theme_dir_path`.
    pub fn load_theme(&self, theme_dir_path: &str) -> io::Result<KvantumThemeData> {
        let theme_name = Path::new(theme_dir_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let kvconfig_path = format!("{}/{}.kvconfig", theme_dir_path, theme_name);
        let svg_path = format!("{}/{}.svg", theme_dir_path, theme_name);

        let config_content = self.file_service.read_file(&kvconfig_path)?;
        let doc = parse_ini(&config_content);
        let config = KvantumConfig::from_ini(&doc);

        let svg_content = if self.file_service.file_exists(&svg_path) {
            Some(self.file_service.read_file(&svg_path)?)
        } else {
            None
        };

        Ok(KvantumThemeData {
            config,
            svg_content,
            theme_name,
            dir_path: theme_dir_path.to_string(),
        })
    }

    /// Save a theme to `theme_dir_path`.
    pub fn save_theme(&self, theme_dir_path: &str, theme: &KvantumThemeData) -> io::Result<()> {
        let theme_name = Path::new(theme_dir_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let kvconfig_path = format!("{}/{}.kvconfig", theme_dir_path, theme_name);

        self.file_service.create_directory(theme_dir_path)?;

        let doc = theme.config.to_ini();
        let content = serialize_ini(&doc);
        self.file_service.write_file(&kvconfig_path, &content)?;

        if let Some(ref svg) = theme.svg_content {
            let svg_path = format!("{}/{}.svg", theme_dir_path, theme_name);
            self.file_service.write_file(&svg_path, svg)?;
        }

        Ok(())
    }

    // --------------------------------------------------------------------------
    // Theme management
    // --------------------------------------------------------------------------

    /// Create a new empty Kvantum theme named `name`.
    pub fn create_theme(&self, name: &str) -> io::Result<String> {
        let theme_dir_path = format!(
            "{}/{}",
            constants::kvantum_config_dir().to_string_lossy(),
            name
        );
        let kvconfig_path = format!("{}/{}.kvconfig", theme_dir_path, name);

        self.file_service.create_directory(&theme_dir_path)?;

        let mut doc = IniDocument {
            header_lines: Vec::new(),
            sections: Vec::new(),
            trailing_lines: Vec::new(),
        };
        doc.set_value("%General", "author", String::new());
        doc.set_value(
            "%General",
            "comment",
            "Created by KEasyDitor".to_string(),
        );
        let content = serialize_ini(&doc);
        self.file_service.write_file(&kvconfig_path, &content)?;

        Ok(theme_dir_path)
    }

    // --------------------------------------------------------------------------
    // Active theme
    // --------------------------------------------------------------------------

    /// Get the currently active Kvantum theme name.
    ///
    /// Reads from `[General]` first (written by `kvantummanager`),
    /// falling back to `[%General]` (legacy format).
    pub fn get_active_theme(&self) -> Option<String> {
        let config_path = format!(
            "{}/kvantum.kvconfig",
            constants::kvantum_config_dir().to_string_lossy()
        );
        if !self.file_service.file_exists(&config_path) {
            return None;
        }
        let content = self.file_service.read_file(&config_path).ok()?;
        let doc = parse_ini(&content);
        doc.get_value("General", "theme")
            .or_else(|| doc.get_value("%General", "theme"))
            .map(|s| s.to_string())
    }

    /// Set the active Kvantum theme.
    pub fn set_active_theme(&self, theme_name: &str) -> io::Result<()> {
        let config_path = format!(
            "{}/kvantum.kvconfig",
            constants::kvantum_config_dir().to_string_lossy()
        );

        let mut doc = if self.file_service.file_exists(&config_path) {
            let content = self.file_service.read_file(&config_path)?;
            parse_ini(&content)
        } else {
            IniDocument {
                header_lines: Vec::new(),
                sections: Vec::new(),
                trailing_lines: Vec::new(),
            }
        };

        doc.set_value("%General", "theme", theme_name.to_string());
        let content = serialize_ini(&doc);
        self.file_service.write_file(&config_path, &content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_theme_tetonoir() {
        let service = KvantumService::new(FileService::new());
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test_fixtures/kvantum/TetoNoir"
        );
        let theme = service.load_theme(fixture_path).unwrap();
        assert_eq!(theme.theme_name, "TetoNoir");
        assert!(theme.svg_content.is_none()); // no SVG in fixture
        assert_eq!(theme.config.general.author(), "Custom");
        assert!(theme.config.general.composite());
    }
}
