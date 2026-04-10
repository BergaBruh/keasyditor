/// Service layer for Klassy window decoration configuration.
///
/// Provides load/save operations for the main `klassyrc` config and the
/// `windecopresetsrc` preset collection, as well as import/export of
/// individual `.klpw` preset files.
use std::io;

use crate::constants;
use crate::ini::{parse_ini, serialize_ini, IniDocument, IniEntry, IniSection};
use crate::models::klassy::{KlassyConfig, KlassyPreset, KlassyPresetCollection};
use crate::services::file_service::FileService;

pub struct KlassyService {
    file_service: FileService,
}

impl KlassyService {
    pub fn new(file_service: FileService) -> Self {
        Self { file_service }
    }

    // --------------------------------------------------------------------------
    // Config (klassyrc)
    // --------------------------------------------------------------------------

    /// Load the main Klassy configuration from `path`, defaulting to
    /// the standard Klassy config path.
    pub fn load_config(&self, path: Option<&str>) -> io::Result<KlassyConfig> {
        let file_path = path
            .map(|s| s.to_string())
            .unwrap_or_else(|| constants::klassy_config_path().to_string_lossy().into_owned());
        let content = self.file_service.read_file(&file_path)?;
        let doc = parse_ini(&content);
        Ok(KlassyConfig::from_ini(&doc))
    }

    /// Save `config` to `path`, defaulting to the standard Klassy config path.
    pub fn save_config(&self, config: &KlassyConfig, path: Option<&str>) -> io::Result<()> {
        let file_path = path
            .map(|s| s.to_string())
            .unwrap_or_else(|| constants::klassy_config_path().to_string_lossy().into_owned());
        let doc = config.to_ini();
        let content = serialize_ini(&doc);
        self.file_service.write_file(&file_path, &content)
    }

    // --------------------------------------------------------------------------
    // Presets (windecopresetsrc)
    // --------------------------------------------------------------------------

    /// Load all presets from the `windecopresetsrc` file.
    pub fn load_presets(&self, path: Option<&str>) -> io::Result<KlassyPresetCollection> {
        let file_path = path
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                constants::klassy_presets_path()
                    .to_string_lossy()
                    .into_owned()
            });
        let content = self.file_service.read_file(&file_path)?;
        let doc = parse_ini(&content);
        Ok(KlassyPresetCollection::from_ini(&doc))
    }

    /// Save presets to the `windecopresetsrc` file.
    pub fn save_presets(
        &self,
        presets: &KlassyPresetCollection,
        path: Option<&str>,
    ) -> io::Result<()> {
        let file_path = path
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                constants::klassy_presets_path()
                    .to_string_lossy()
                    .into_owned()
            });
        let doc = presets.to_ini();
        let content = serialize_ini(&doc);
        self.file_service.write_file(&file_path, &content)
    }

    // --------------------------------------------------------------------------
    // .klpw import / export
    // --------------------------------------------------------------------------

    /// The header section name used in `.klpw` preset files.
    const KLPW_HEADER_SECTION: &'static str = "Klassy Window Decoration Preset File";

    /// Import a single preset from a `.klpw` file.
    pub fn import_preset(&self, klpw_path: &str) -> io::Result<KlassyPreset> {
        let content = self.file_service.read_file(klpw_path)?;
        let doc = parse_ini(&content);

        const PRESET_PREFIX: &str = "Windeco Preset ";

        let preset_section = doc
            .sections
            .iter()
            .find(|s| s.name != Self::KLPW_HEADER_SECTION && s.name.starts_with(PRESET_PREFIX));

        let preset_section = preset_section.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Invalid .klpw file: no [Windeco Preset <name>] section found in \"{}\"",
                    klpw_path
                ),
            )
        })?;

        let preset_name = preset_section.name[PRESET_PREFIX.len()..].to_string();
        let mut values = std::collections::HashMap::new();
        for entry in &preset_section.entries {
            if !entry.key.is_empty() {
                values.insert(entry.key.clone(), entry.value.clone());
            }
        }

        Ok(KlassyPreset::new(preset_name, values))
    }

    /// Export a preset to a `.klpw` file.
    pub fn export_preset(&self, preset: &KlassyPreset, output_path: &str) -> io::Result<()> {
        let mut doc = IniDocument {
            header_lines: Vec::new(),
            sections: Vec::new(),
            trailing_lines: Vec::new(),
        };

        // Header section
        doc.sections.push(IniSection {
            name: Self::KLPW_HEADER_SECTION.to_string(),
            entries: vec![IniEntry {
                key: "Version".to_string(),
                value: "1".to_string(),
                comment: None,
            }],
            preceding_lines: Vec::new(),
        });

        // Preset data section
        let section_name = format!("Windeco Preset {}", preset.name);
        let map = preset.to_map();
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();
        let entries: Vec<IniEntry> = keys
            .iter()
            .map(|k| IniEntry {
                key: (*k).clone(),
                value: map[*k].clone(),
                comment: None,
            })
            .collect();

        doc.sections.push(IniSection {
            name: section_name,
            entries,
            preceding_lines: Vec::new(),
        });

        let content = serialize_ini(&doc);
        self.file_service.write_file(output_path, &content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_from_fixture() {
        let service = KlassyService::new(FileService::new());
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test_fixtures/klassy/sample_klassyrc"
        );
        let config = service.load_config(Some(fixture_path)).unwrap();
        assert_eq!(config.window_corner_radius(), Some(8));
        assert_eq!(
            config.button_shape(),
            Some(crate::models::klassy::ButtonShape::FullHeightRectangle)
        );
    }
}
