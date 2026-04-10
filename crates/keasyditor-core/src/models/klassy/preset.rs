/// Data model for Klassy window-decoration presets (`windecopresetsrc`).
///
/// Each preset is a named INI section containing up to ~179 key-value pairs
/// that fully describe a window decoration style.
use std::collections::HashMap;

use crate::ini::{IniDocument, IniEntry, IniSection};
use crate::models::klassy::config::KlassyConfig;

/// A single named preset from `windecopresetsrc`.
#[derive(Clone, Debug, PartialEq)]
pub struct KlassyPreset {
    /// Display name of the preset (the INI section name).
    pub name: String,
    /// All key-value pairs for this preset.
    values: HashMap<String, String>,
}

impl KlassyPreset {
    /// Create a new preset with the given name and values.
    pub fn new(name: String, values: HashMap<String, String>) -> Self {
        Self { name, values }
    }

    /// Whether this is a bundled (read-only) preset shipped with Klassy.
    pub fn is_bundled(&self) -> bool {
        self.values.get("BundledPreset").is_some_and(|v| v == "true")
    }

    /// Get a raw string value by key, or `None` if absent.
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|v| v.as_str())
    }

    /// Set a raw string value by key.
    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    /// Remove a key. Returns true if it existed.
    pub fn remove_value(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }

    /// The number of key-value pairs in this preset.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the preset is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// All keys in this preset.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.values.keys().map(|k| k.as_str())
    }

    /// Return a copy of all key-value pairs.
    pub fn to_map(&self) -> HashMap<String, String> {
        self.values.clone()
    }

    /// Apply this preset's values to a config, writing all keys into the
    /// appropriate sections.
    pub fn apply_to_config(&self, config: &KlassyConfig) -> KlassyConfig {
        let mut result = config.clone();
        for (key, value) in &self.values {
            let section = section_for_key(key);
            result.set_value(section, key, value.clone());
        }
        result
    }
}

impl std::fmt::Display for KlassyPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KlassyPreset(\"{}\", {} keys, bundled={})",
            self.name,
            self.values.len(),
            self.is_bundled()
        )
    }
}

/// Determine the config section a preset key belongs in.
fn section_for_key(key: &str) -> &'static str {
    // ShadowStyle
    if key == "ShadowColor" || key == "ShadowSize" || key == "ShadowStrength" {
        return "ShadowStyle";
    }

    // ButtonColors
    if key.starts_with("ButtonBackgroundColors")
        || key.starts_with("ButtonBackgroundOpacity")
        || key == "LockButtonColorsActiveInactive"
        || key == "ButtonColorsInactiveSameHoverPress"
        || key.starts_with("ButtonIconColors")
        || key.starts_with("ButtonIconOpacity")
        || key.starts_with("ButtonOverrideColors")
        || key.starts_with("CloseButtonIconColor")
        || key.starts_with("CloseButtonCustomIconColor")
        || key.starts_with("MaximizeButtonIconColor")
        || key.starts_with("MaximizeButtonCustomIconColor")
        || key.starts_with("MinimizeButtonIconColor")
        || key.starts_with("MinimizeButtonCustomIconColor")
        || key.starts_with("NegativeCloseBackground")
        || key.starts_with("ShowBackground")
        || key.starts_with("ShowCloseBackground")
        || key.starts_with("ShowCloseIcon")
        || key.starts_with("ShowCloseOutline")
        || key.starts_with("ShowIcon")
        || key.starts_with("ShowOutline")
        || key.starts_with("VaryColor")
        || key.starts_with("UseHoverAccent")
    {
        return "ButtonColors";
    }

    // TitleBarOpacity
    if key == "ActiveTitleBarOpacity"
        || key == "InactiveTitleBarOpacity"
        || key == "OpaqueMaximizedTitleBars"
        || key == "OverrideActiveTitleBarOpacity"
        || key == "OverrideInactiveTitleBarOpacity"
        || key == "ApplyOpacityToHeader"
    {
        return "TitleBarOpacity";
    }

    // TitleBarSpacing
    if key == "TitleBarTopMargin"
        || key == "TitleBarBottomMargin"
        || key == "TitleBarLeftMargin"
        || key == "TitleBarRightMargin"
        || key == "PercentMaximizedTopBottomMargins"
        || key.starts_with("LockTitleBar")
    {
        return "TitleBarSpacing";
    }

    // WindowOutlineStyle
    if key.starts_with("WindowOutlineStyle")
        || key.starts_with("WindowOutlineCustomColor")
        || key == "WindowOutlineOverlap"
        || key.starts_with("LockWindowOutline")
        || key.starts_with("WindowOutlineAccentColor")
        || key.starts_with("WindowOutlineAccentWithContrast")
        || key.starts_with("WindowOutlineContrastOpacity")
        || key.starts_with("WindowOutlineCustomWithContrast")
        || key == "WindowOutlineShadowColorOpacity"
        || key == "WindowOutlineSnapToWholePixel"
        || key == "WindowOutlineThickness"
    {
        return "WindowOutlineStyle";
    }

    // Style
    if key == "ButtonGradient" || key == "ScrollBarSeparator" || key == "DrawBackgroundGradient" {
        return "Style";
    }

    // Default: Windeco
    "Windeco"
}

/// Collection of all presets from `windecopresetsrc`.
#[derive(Clone, Debug, PartialEq)]
pub struct KlassyPresetCollection {
    pub presets: Vec<KlassyPreset>,
}

impl KlassyPresetCollection {
    /// Create a new collection.
    pub fn new(presets: Vec<KlassyPreset>) -> Self {
        Self { presets }
    }

    /// Parse all presets from a `windecopresetsrc` `IniDocument`.
    pub fn from_ini(doc: &IniDocument) -> Self {
        let mut presets = Vec::new();
        for sec in &doc.sections {
            let mut values = HashMap::new();
            for entry in &sec.entries {
                if !entry.key.is_empty() {
                    values.insert(entry.key.clone(), entry.value.clone());
                }
            }
            presets.push(KlassyPreset {
                name: sec.name.clone(),
                values,
            });
        }
        Self { presets }
    }

    /// Serialize all presets back to an `IniDocument`.
    pub fn to_ini(&self) -> IniDocument {
        let mut doc = IniDocument {
            header_lines: Vec::new(),
            sections: Vec::new(),
            trailing_lines: Vec::new(),
        };
        for preset in &self.presets {
            let mut entries = Vec::new();
            let mut keys: Vec<&String> = preset.values.keys().collect();
            keys.sort();
            for key in keys {
                entries.push(IniEntry {
                    key: key.clone(),
                    value: preset.values[key].clone(),
                    comment: None,
                });
            }
            doc.sections.push(IniSection {
                name: preset.name.clone(),
                entries,
                preceding_lines: Vec::new(),
            });
        }
        doc
    }

    /// Find a preset by name, or `None` if not found.
    pub fn get_preset(&self, name: &str) -> Option<&KlassyPreset> {
        self.presets.iter().find(|p| p.name == name)
    }

    /// All preset names.
    pub fn names(&self) -> Vec<&str> {
        self.presets.iter().map(|p| p.name.as_str()).collect()
    }

    /// Number of presets.
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    /// Whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.presets.is_empty()
    }
}

impl std::fmt::Display for KlassyPresetCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KlassyPresetCollection({} presets)", self.presets.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ini::parse_ini;

    #[test]
    fn preset_from_ini() {
        let input = "[MyPreset]\nButtonShape=ShapeSmallCircle\nBundledPreset=true\n";
        let doc = parse_ini(input);
        let collection = KlassyPresetCollection::from_ini(&doc);

        assert_eq!(collection.len(), 1);
        let preset = collection.get_preset("MyPreset").unwrap();
        assert_eq!(preset.get_value("ButtonShape"), Some("ShapeSmallCircle"));
        assert!(preset.is_bundled());
    }

    #[test]
    fn preset_not_bundled() {
        let input = "[Custom]\nButtonShape=ShapeSmallCircle\n";
        let doc = parse_ini(input);
        let collection = KlassyPresetCollection::from_ini(&doc);
        let preset = collection.get_preset("Custom").unwrap();
        assert!(!preset.is_bundled());
    }

    #[test]
    fn section_routing() {
        assert_eq!(section_for_key("ShadowColor"), "ShadowStyle");
        assert_eq!(section_for_key("ShadowSize"), "ShadowStyle");
        assert_eq!(
            section_for_key("ButtonBackgroundColorsActive"),
            "ButtonColors"
        );
        assert_eq!(
            section_for_key("ActiveTitleBarOpacity"),
            "TitleBarOpacity"
        );
        assert_eq!(section_for_key("TitleBarTopMargin"), "TitleBarSpacing");
        assert_eq!(
            section_for_key("WindowOutlineStyleActive"),
            "WindowOutlineStyle"
        );
        assert_eq!(section_for_key("ButtonGradient"), "Style");
        assert_eq!(section_for_key("ButtonShape"), "Windeco");
        assert_eq!(section_for_key("SomeUnknownKey"), "Windeco");
    }

    #[test]
    fn apply_to_config() {
        let config = KlassyConfig::new(HashMap::new());
        let mut values = HashMap::new();
        values.insert("ButtonShape".to_string(), "ShapeSmallCircle".to_string());
        values.insert("ShadowSize".to_string(), "ShadowLarge".to_string());
        values.insert("ButtonGradient".to_string(), "true".to_string());

        let preset = KlassyPreset::new("Test".to_string(), values);
        let result = preset.apply_to_config(&config);

        assert_eq!(
            result.get_value("Windeco", "ButtonShape"),
            Some("ShapeSmallCircle")
        );
        assert_eq!(
            result.get_value("ShadowStyle", "ShadowSize"),
            Some("ShadowLarge")
        );
        assert_eq!(
            result.get_value("Style", "ButtonGradient"),
            Some("true")
        );
    }

    #[test]
    fn collection_round_trip() {
        let input = "[A]\nk1=v1\n[B]\nk2=v2\n";
        let doc = parse_ini(input);
        let collection = KlassyPresetCollection::from_ini(&doc);

        assert_eq!(collection.len(), 2);
        assert_eq!(collection.names(), vec!["A", "B"]);

        let doc2 = collection.to_ini();
        let collection2 = KlassyPresetCollection::from_ini(&doc2);
        assert_eq!(collection, collection2);
    }
}
