/// Full Kvantum theme configuration model.
///
/// Combines `KvantumGeneral`, `KvantumColors`, hacks, and all widget sections
/// into a single model that can be serialized to/from an `IniDocument`.
use std::collections::{HashMap, HashSet};

use crate::ini::{IniDocument, IniEntry, IniSection};
use crate::models::kvantum::colors::KvantumColors;
use crate::models::kvantum::general::KvantumGeneral;
use crate::models::kvantum::section::KvantumSection;

/// Complete model for a Kvantum `.kvconfig` file.
#[derive(Clone, Debug, PartialEq)]
pub struct KvantumConfig {
    /// The `[%General]` section.
    pub general: KvantumGeneral,
    /// The `[GeneralColors]` section.
    pub colors: KvantumColors,
    /// The `[Hacks]` section as raw key-value pairs.
    pub hacks: HashMap<String, String>,
    /// All widget sections keyed by section name.
    /// Does not include `%General`, `GeneralColors`, or `Hacks`.
    pub widget_sections: HashMap<String, KvantumSection>,
    /// Preserve section ordering for round-trip fidelity.
    section_order: Vec<String>,
}

impl KvantumConfig {
    /// Create a new KvantumConfig.
    pub fn new(
        general: KvantumGeneral,
        colors: KvantumColors,
        hacks: HashMap<String, String>,
        widget_sections: HashMap<String, KvantumSection>,
    ) -> Self {
        let section_order: Vec<String> = widget_sections.keys().cloned().collect();
        Self {
            general,
            colors,
            hacks,
            widget_sections,
            section_order,
        }
    }

    // --------------------------------------------------------------------------
    // Section access
    // --------------------------------------------------------------------------

    /// Get a widget section by name, or `None` if not found.
    pub fn get_section(&self, name: &str) -> Option<&KvantumSection> {
        self.widget_sections.get(name)
    }

    /// Resolve a widget section by following its inheritance chain.
    ///
    /// For example, `resolve_section("ComboBox")` merges:
    ///   `PanelButtonCommand` -> `LineEdit` -> `ComboBox`
    ///
    /// Returns `None` if the section does not exist.
    pub fn resolve_section(&self, name: &str) -> Option<KvantumSection> {
        let section = self.widget_sections.get(name)?;

        // Build the inheritance chain from leaf to root.
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        let mut current = Some(section);

        while let Some(sec) = current {
            if visited.contains(&sec.name) {
                // Circular inheritance -- break to avoid infinite loop.
                break;
            }
            visited.insert(sec.name.clone());
            chain.push(sec);
            current = sec
                .inherits()
                .and_then(|parent_name| self.widget_sections.get(parent_name));
        }

        // Reverse so root ancestor is first.
        chain.reverse();

        // Merge from root to leaf.
        let mut resolved = chain[0].clone();
        for sec in &chain[1..] {
            resolved = sec.resolve_with(&resolved);
        }

        Some(resolved)
    }

    /// Return all section names (widget sections only).
    pub fn section_names(&self) -> Vec<&str> {
        self.section_order.iter().map(|s| s.as_str()).collect()
    }

    // --------------------------------------------------------------------------
    // Deserialization
    // --------------------------------------------------------------------------

    /// Parse a `KvantumConfig` from an `IniDocument`.
    pub fn from_ini(doc: &IniDocument) -> Self {
        let mut general_map = HashMap::new();
        let mut colors_map = HashMap::new();
        let mut hacks_map = HashMap::new();
        let mut widgets = HashMap::new();
        let mut section_order = Vec::new();

        for sec in &doc.sections {
            let mut map = HashMap::new();
            for entry in &sec.entries {
                if !entry.key.is_empty() {
                    map.insert(entry.key.clone(), entry.value.clone());
                }
            }

            match sec.name.as_str() {
                "%General" => general_map = map,
                "GeneralColors" => colors_map = map,
                "Hacks" => hacks_map = map,
                name => {
                    section_order.push(name.to_string());
                    widgets.insert(
                        name.to_string(),
                        KvantumSection::from_map(name, map),
                    );
                }
            }
        }

        Self {
            general: KvantumGeneral::from_map(general_map),
            colors: KvantumColors::from_map(colors_map),
            hacks: hacks_map,
            widget_sections: widgets,
            section_order,
        }
    }

    // --------------------------------------------------------------------------
    // Serialization
    // --------------------------------------------------------------------------

    /// Convert this config back to an `IniDocument`.
    pub fn to_ini(&self) -> IniDocument {
        let mut doc = IniDocument {
            header_lines: Vec::new(),
            sections: Vec::new(),
            trailing_lines: Vec::new(),
        };

        // %General
        add_section(&mut doc, "%General", &self.general.to_map());

        // GeneralColors
        add_section(&mut doc, "GeneralColors", &self.colors.to_map());

        // Hacks
        add_section(&mut doc, "Hacks", &self.hacks);

        // Widget sections in preserved order
        for name in &self.section_order {
            if let Some(sec) = self.widget_sections.get(name) {
                add_section(&mut doc, name, &sec.to_map());
            }
        }

        doc
    }

    // --------------------------------------------------------------------------
    // Immutable updates
    // --------------------------------------------------------------------------

    /// Return a copy with replaced `KvantumColors`.
    pub fn copy_with_colors(&self, new_colors: KvantumColors) -> Self {
        Self {
            general: self.general.clone(),
            colors: new_colors,
            hacks: self.hacks.clone(),
            widget_sections: self.widget_sections.clone(),
            section_order: self.section_order.clone(),
        }
    }

    /// Return a copy with replaced `KvantumGeneral`.
    pub fn copy_with_general(&self, new_general: KvantumGeneral) -> Self {
        Self {
            general: new_general,
            colors: self.colors.clone(),
            hacks: self.hacks.clone(),
            widget_sections: self.widget_sections.clone(),
            section_order: self.section_order.clone(),
        }
    }

    /// Return a copy with one widget section added or replaced.
    pub fn copy_with_section(&self, name: &str, section: KvantumSection) -> Self {
        let mut new_sections = self.widget_sections.clone();
        let mut new_order = self.section_order.clone();
        if !new_sections.contains_key(name) {
            new_order.push(name.to_string());
        }
        new_sections.insert(name.to_string(), section);
        Self {
            general: self.general.clone(),
            colors: self.colors.clone(),
            hacks: self.hacks.clone(),
            widget_sections: new_sections,
            section_order: new_order,
        }
    }

    /// Return a copy with replaced hacks.
    pub fn copy_with_hacks(&self, new_hacks: HashMap<String, String>) -> Self {
        Self {
            general: self.general.clone(),
            colors: self.colors.clone(),
            hacks: new_hacks,
            widget_sections: self.widget_sections.clone(),
            section_order: self.section_order.clone(),
        }
    }
}

fn add_section(doc: &mut IniDocument, name: &str, map: &HashMap<String, String>) {
    let mut entries = Vec::new();
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    for key in keys {
        entries.push(IniEntry {
            key: key.clone(),
            value: map[key].clone(),
            comment: None,
        });
    }
    doc.sections.push(IniSection {
        name: name.to_string(),
        entries,
        preceding_lines: Vec::new(),
    });
}

impl std::fmt::Display for KvantumConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KvantumConfig(general: {} keys, colors: {} keys, hacks: {} keys, sections: {})",
            self.general.len(),
            self.colors.len(),
            self.hacks.len(),
            self.widget_sections.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;
    use crate::ini::{parse_ini, serialize_ini};

    #[test]
    fn from_ini_minimal() {
        let fixture = include_str!("../../../../../test_fixtures/kvantum/minimal.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        assert_eq!(config.general.author(), "Test");
        assert!(config.general.composite());
        assert_eq!(config.colors.window_color(), Rgba::rgb(0, 0, 0));
        assert_eq!(config.colors.text_color(), Rgba::rgb(0xff, 0xff, 0xff));
        assert_eq!(config.widget_sections.len(), 1);
        assert!(config.widget_sections.contains_key("PanelButtonCommand"));

        let pbc = config.get_section("PanelButtonCommand").unwrap();
        assert_eq!(pbc.frame(), Some(true));
        assert_eq!(pbc.text_normal_color(), Some("#cccccc"));
    }

    #[test]
    fn from_ini_tetonoir() {
        let fixture =
            include_str!("../../../../../test_fixtures/kvantum/TetoNoir/TetoNoir.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        assert_eq!(config.general.author(), "Custom");
        assert!(config.general.composite());
        assert!(config.general.translucent_windows());
        assert!(config.general.blurring());
        assert_eq!(config.general.progressbar_thickness(), 8);
        assert_eq!(config.general.reduce_window_opacity(), 67);

        // Colors
        assert_eq!(config.colors.window_color(), Rgba::rgb(0x1a, 0x11, 0x11));
        assert_eq!(config.colors.highlight_color(), Rgba::rgb(0x73, 0x33, 0x30));
        assert_eq!(
            config.colors.highlight_text_color(),
            Rgba::rgb(0xff, 0xff, 0xff)
        );

        // Hacks
        assert_eq!(config.hacks.get("respect_darkness"), Some(&"true".to_string()));
        assert_eq!(
            config.hacks.get("transparent_dolphin_view"),
            Some(&"true".to_string())
        );

        // Widget sections
        assert!(config.widget_sections.len() > 20);
        let pbc = config.get_section("PanelButtonCommand").unwrap();
        assert_eq!(pbc.frame(), Some(true));
        assert_eq!(pbc.frame_top(), Some(3));
        assert_eq!(pbc.text_normal_color(), Some("#d4d4d8"));
        assert_eq!(pbc.text_focus_color(), Some("#e05555"));

        // Inheritance
        let pbt = config.get_section("PanelButtonTool").unwrap();
        assert_eq!(pbt.inherits(), Some("PanelButtonCommand"));
    }

    #[test]
    fn resolve_section_basic() {
        let fixture =
            include_str!("../../../../../test_fixtures/kvantum/TetoNoir/TetoNoir.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        // PanelButtonTool inherits PanelButtonCommand
        let resolved = config.resolve_section("PanelButtonTool").unwrap();
        assert_eq!(resolved.frame(), Some(true)); // from parent
        assert_eq!(resolved.frame_top(), Some(3)); // from parent
        assert_eq!(resolved.text_normal_color(), Some("#d4d4d8")); // from parent
        assert!(resolved.inherits().is_none()); // resolved
    }

    #[test]
    fn resolve_section_chain() {
        let fixture =
            include_str!("../../../../../test_fixtures/kvantum/TetoNoir/TetoNoir.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        // ComboBox -> LineEdit -> PanelButtonCommand
        let resolved = config.resolve_section("ComboBox").unwrap();
        assert_eq!(resolved.frame(), Some(true)); // from PanelButtonCommand
        assert_eq!(resolved.text_margin_left(), Some(6)); // overridden by ComboBox
        assert_eq!(resolved.text_margin_right(), Some(6)); // overridden by ComboBox
        // from LineEdit (overrides PanelButtonCommand's 3)
        assert_eq!(resolved.frame_top(), Some(4));
    }

    #[test]
    fn resolve_section_circular_protection() {
        // Create a circular inheritance scenario
        let mut widgets = HashMap::new();
        let mut a_map = HashMap::new();
        a_map.insert("inherits".to_string(), "B".to_string());
        a_map.insert("x".to_string(), "1".to_string());
        widgets.insert("A".to_string(), KvantumSection::from_map("A", a_map));

        let mut b_map = HashMap::new();
        b_map.insert("inherits".to_string(), "A".to_string());
        b_map.insert("y".to_string(), "2".to_string());
        widgets.insert("B".to_string(), KvantumSection::from_map("B", b_map));

        let config = KvantumConfig::new(
            KvantumGeneral::empty(),
            KvantumColors::empty(),
            HashMap::new(),
            widgets,
        );

        // Should not panic or infinite loop
        let resolved = config.resolve_section("A").unwrap();
        assert_eq!(resolved.get_value("x"), Some("1"));
    }

    #[test]
    fn round_trip_minimal() {
        let fixture = include_str!("../../../../../test_fixtures/kvantum/minimal.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        let doc2 = config.to_ini();
        let config2 = KvantumConfig::from_ini(&doc2);

        assert_eq!(config.general, config2.general);
        assert_eq!(config.colors, config2.colors);
        assert_eq!(config.hacks, config2.hacks);
        assert_eq!(config.widget_sections, config2.widget_sections);
    }

    #[test]
    fn round_trip_tetonoir() {
        let fixture =
            include_str!("../../../../../test_fixtures/kvantum/TetoNoir/TetoNoir.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        let doc2 = config.to_ini();
        let config2 = KvantumConfig::from_ini(&doc2);

        // Verify structural equality
        assert_eq!(config.general, config2.general);
        assert_eq!(config.colors, config2.colors);
        assert_eq!(config.hacks, config2.hacks);
        assert_eq!(config.widget_sections.len(), config2.widget_sections.len());
        for (name, sec) in &config.widget_sections {
            let sec2 = config2.widget_sections.get(name).unwrap();
            assert_eq!(sec, sec2, "Section mismatch: {}", name);
        }
    }

    #[test]
    fn round_trip_serialized_reparseable() {
        let fixture =
            include_str!("../../../../../test_fixtures/kvantum/TetoNoir/TetoNoir.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);
        let doc2 = config.to_ini();
        let serialized = serialize_ini(&doc2);

        // The serialized output should be reparseable
        let doc3 = parse_ini(&serialized);
        let config3 = KvantumConfig::from_ini(&doc3);

        assert_eq!(config.general, config3.general);
        assert_eq!(config.colors, config3.colors);
    }

    #[test]
    fn copy_with_methods() {
        let fixture = include_str!("../../../../../test_fixtures/kvantum/minimal.kvconfig");
        let doc = parse_ini(fixture);
        let config = KvantumConfig::from_ini(&doc);

        // copy_with_colors
        let mut new_colors = config.colors.clone();
        new_colors.set_window_color(Rgba::rgb(0xff, 0x00, 0x00));
        let config2 = config.copy_with_colors(new_colors);
        assert_eq!(config2.colors.window_color(), Rgba::rgb(0xff, 0x00, 0x00));
        assert_eq!(config2.general, config.general); // unchanged

        // copy_with_general
        let mut new_general = config.general.clone();
        new_general.set_author("NewAuthor");
        let config3 = config.copy_with_general(new_general);
        assert_eq!(config3.general.author(), "NewAuthor");

        // copy_with_hacks
        let mut new_hacks = HashMap::new();
        new_hacks.insert("test".to_string(), "true".to_string());
        let config4 = config.copy_with_hacks(new_hacks);
        assert_eq!(config4.hacks.get("test"), Some(&"true".to_string()));
    }
}
