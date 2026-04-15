/// Service layer for Kvantum theme management.
///
/// Handles loading, saving, creating, and cloning Kvantum themes, as well as
/// reading and writing the global active-theme setting.
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants;
use crate::ini::{parse_ini, serialize_ini, IniDocument};
use crate::models::kvantum::KvantumConfig;
use crate::services::file_service::FileService;

/// Default palette written into new themes' `[GeneralColors]` section.
/// Mirrors upstream KvFlat so a freshly-created theme renders consistently
/// under both the internal preview and the real Kvantum Qt plugin.
const DEFAULT_GENERAL_COLORS: &[(&str, &str)] = &[
    ("window.color", "#3D3D3E"),
    ("base.color", "#2E2E2E"),
    ("alt.base.color", "#383838"),
    ("button.color", "#555555"),
    ("light.color", "#626262"),
    ("mid.light.color", "#555555"),
    ("dark.color", "#171717"),
    ("mid.color", "#3C3C3C"),
    ("highlight.color", "#3F67A5"),
    ("inactive.highlight.color", "#2E4C7A"),
    ("text.color", "#FFFFFF"),
    ("window.text.color", "#FFFFFF"),
    ("button.text.color", "#FFFFFF"),
    ("disabled.text.color", "#A0A0A0"),
    ("tooltip.text.color", "#FFFFFF"),
    ("highlight.text.color", "#FFFFFF"),
    ("link.color", "#2EB8E6"),
    ("link.visited.color", "#FF6666"),
    ("progress.indicator.text.color", "#FFFFFF"),
];

/// Walk `system_dirs` looking for `<subdir>/<subdir>.svg` pairs and return
/// the first match (alphabetical by subdirectory name). Falls back to the
/// bundled `KvFlat.svg` if nothing is found.
fn find_base_template_svg_in(system_dirs: &[PathBuf], file_svc: &FileService) -> Option<String> {
    let mut candidates: BTreeMap<String, String> = BTreeMap::new();
    for dir in system_dirs {
        let dir_str = dir.to_string_lossy();
        let entries = match file_svc.list_directory(&dir_str) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry_path in entries {
            if !file_svc.directory_exists(&entry_path) {
                continue;
            }
            let name = match Path::new(&entry_path).file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            let svg_path = format!("{}/{}.svg", entry_path, name);
            if !file_svc.file_exists(&svg_path) {
                continue;
            }
            candidates.entry(name).or_insert(svg_path);
        }
    }
    if let Some((_, svg_path)) = candidates.into_iter().next()
        && let Ok(content) = file_svc.read_file(&svg_path)
    {
        return Some(content);
    }
    Some(include_str!("../../assets/KvFlat.svg").to_string())
}

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

    fn find_base_template_svg(&self) -> Option<String> {
        find_base_template_svg_in(&[constants::kvantum_system_dir()], &self.file_service)
    }

    fn create_theme_at(&self, base_dir: &Path, name: &str) -> io::Result<String> {
        let theme_dir = base_dir.join(name);
        let theme_dir_str = theme_dir.to_string_lossy().into_owned();
        let kvconfig_path = format!("{}/{}.kvconfig", theme_dir_str, name);

        self.file_service.create_directory(&theme_dir_str)?;

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
        for (key, value) in DEFAULT_GENERAL_COLORS {
            doc.set_value("GeneralColors", key, (*value).to_string());
        }
        let content = serialize_ini(&doc);
        self.file_service.write_file(&kvconfig_path, &content)?;

        if let Some(svg) = self.find_base_template_svg() {
            let svg_path = format!("{}/{}.svg", theme_dir_str, name);
            self.file_service.write_file(&svg_path, &svg)?;
        }

        Ok(theme_dir_str)
    }

    /// Create a new empty Kvantum theme named `name`.
    pub fn create_theme(&self, name: &str) -> io::Result<String> {
        self.create_theme_at(&constants::kvantum_config_dir(), name)
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

    #[test]
    fn find_base_template_svg_in_picks_first_alphabetical() {
        let fixture = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test_fixtures/kvantum_system_fake"
        ));
        let svc = FileService::new();
        let result = find_base_template_svg_in(&[fixture], &svc);
        let content = result.expect("should return Some");
        // FakeAlpha is alphabetically first AND has an svg sibling.
        // FakeBeta has no svg (skipped). FakeGamma is later alphabetically.
        assert!(
            content.contains("#aabbcc"),
            "expected FakeAlpha svg (containing #aabbcc), got: {}",
            content
        );
        assert!(
            !content.contains("#112233"),
            "should NOT pick FakeGamma"
        );
    }

    #[test]
    fn find_base_template_svg_in_falls_back_to_bundled() {
        let fixture = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test_fixtures/kvantum_system_empty"
        ));
        let svc = FileService::new();
        let result = find_base_template_svg_in(&[fixture], &svc);
        let content = result.expect("should return bundled fallback");
        let bundled = include_str!("../../assets/KvFlat.svg");
        assert_eq!(content, bundled);
    }

    #[test]
    fn find_base_template_svg_in_handles_missing_dir() {
        let svc = FileService::new();
        let result = find_base_template_svg_in(
            &[PathBuf::from("/nonexistent/kvantum/path/xyz")],
            &svc,
        );
        let content = result.expect("should return bundled fallback, not panic");
        let bundled = include_str!("../../assets/KvFlat.svg");
        assert_eq!(content, bundled);
    }

    #[test]
    fn create_theme_at_writes_kvconfig_and_svg() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let svc = KvantumService::new(FileService::new());
        let theme_path = svc
            .create_theme_at(tmp.path(), "TestTheme")
            .expect("create_theme_at should succeed");

        let kvconfig = format!("{}/TestTheme.kvconfig", theme_path);
        let svg = format!("{}/TestTheme.svg", theme_path);

        let fs = FileService::new();
        assert!(fs.file_exists(&kvconfig), "kvconfig should be written");
        assert!(fs.file_exists(&svg), "svg should be written");

        let kvconfig_content = fs.read_file(&kvconfig).unwrap();
        assert!(
            kvconfig_content.contains("[%General]"),
            "kvconfig should contain [%General] section"
        );
        assert!(
            kvconfig_content.contains("[GeneralColors]"),
            "kvconfig should contain [GeneralColors] section"
        );
        assert!(
            kvconfig_content.contains("highlight.color=#3F67A5"),
            "kvconfig should contain default highlight color"
        );

        let svg_content = fs.read_file(&svg).unwrap();
        assert!(!svg_content.is_empty(), "svg should not be empty");
        assert!(
            svg_content.contains("<svg") || svg_content.contains("<?xml"),
            "svg should look like svg/xml content"
        );
    }

    #[test]
    fn bundled_kvflat_svg_is_valid() {
        let bundled = include_str!("../../assets/KvFlat.svg");
        assert!(!bundled.is_empty(), "bundled svg should not be empty");
        assert!(
            bundled.contains("<svg"),
            "bundled svg should contain an <svg tag"
        );
        let ids = crate::svg::get_all_element_ids(bundled);
        assert!(
            !ids.is_empty(),
            "bundled svg should parse into a non-empty id list"
        );
    }
}
