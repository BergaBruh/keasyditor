/// Data model for the Klassy main configuration file (`klassyrc`).
///
/// Uses a generic `HashMap<String, HashMap<String, String>>` internally
/// (section -> key -> value) so it can handle any key Klassy introduces
/// without code changes. Typed convenience getters/setters are provided
/// for the most commonly accessed fields.
use std::collections::HashMap;

use crate::color::{self, Rgba};
use crate::ini::{IniDocument, IniEntry, IniSection};
use crate::models::klassy::enums::*;

/// Complete model for a Klassy `klassyrc` configuration file.
#[derive(Clone, Debug, PartialEq)]
pub struct KlassyConfig {
    sections: HashMap<String, HashMap<String, String>>,
    /// Preserve section ordering for round-trip fidelity.
    section_order: Vec<String>,
}

impl KlassyConfig {
    /// Create from a pre-built sections map.
    pub fn new(sections: HashMap<String, HashMap<String, String>>) -> Self {
        let section_order: Vec<String> = sections.keys().cloned().collect();
        Self {
            sections,
            section_order,
        }
    }

    // ---- Generic access --------------------------------------------------------

    /// Get a raw string value from section/key, or `None` if absent.
    pub fn get_value(&self, section: &str, key: &str) -> Option<&str> {
        self.sections
            .get(section)
            .and_then(|s| s.get(key).map(|v| v.as_str()))
    }

    /// Set a raw string value in section/key. Creates the section if needed.
    pub fn set_value(&mut self, section: &str, key: &str, value: String) {
        let sec = self.sections.entry(section.to_string()).or_insert_with(|| {
            self.section_order.push(section.to_string());
            HashMap::new()
        });
        sec.insert(key.to_string(), value);
    }

    /// Remove a key from a section. Returns true if it existed.
    pub fn remove_value(&mut self, section: &str, key: &str) -> bool {
        if let Some(sec) = self.sections.get_mut(section) {
            sec.remove(key).is_some()
        } else {
            false
        }
    }

    /// Return a copy of all key-value pairs in a section.
    pub fn get_section(&self, section: &str) -> HashMap<String, String> {
        self.sections
            .get(section)
            .cloned()
            .unwrap_or_default()
    }

    /// Return the list of section names.
    pub fn section_names(&self) -> Vec<&str> {
        self.section_order.iter().map(|s| s.as_str()).collect()
    }

    /// Whether a section exists and contains a key.
    pub fn has_value(&self, section: &str, key: &str) -> bool {
        self.sections
            .get(section)
            .is_some_and(|s| s.contains_key(key))
    }

    /// Return the full internal map (deep copy).
    pub fn to_map(&self) -> HashMap<String, HashMap<String, String>> {
        self.sections.clone()
    }

    // ---- Typed helpers (private) ------------------------------------------------

    fn get_int(&self, section: &str, key: &str) -> Option<i64> {
        self.get_value(section, key)?.parse().ok()
    }

    fn get_f64(&self, section: &str, key: &str) -> Option<f64> {
        self.get_value(section, key)?.parse().ok()
    }

    fn get_bool(&self, section: &str, key: &str) -> Option<bool> {
        Some(self.get_value(section, key)? == "true")
    }

    fn get_color(&self, section: &str, key: &str) -> Option<Rgba> {
        color::try_parse(self.get_value(section, key)?)
    }

    fn set_int(&mut self, section: &str, key: &str, value: i64) {
        self.set_value(section, key, value.to_string());
    }

    fn set_f64(&mut self, section: &str, key: &str, value: f64) {
        self.set_value(section, key, value.to_string());
    }

    fn set_bool(&mut self, section: &str, key: &str, value: bool) {
        self.set_value(section, key, value.to_string());
    }

    fn set_color(&mut self, section: &str, key: &str, value: Rgba) {
        self.set_value(section, key, color::to_klassy_color(&value));
    }

    // ---- Typed convenience getters / setters ------------------------------------

    // -- Windeco section --

    pub fn button_shape(&self) -> Option<ButtonShape> {
        ButtonShape::from_value(self.get_value("Windeco", "ButtonShape")?)
    }

    pub fn set_button_shape(&mut self, shape: ButtonShape) {
        self.set_value("Windeco", "ButtonShape", shape.value().to_string());
    }

    pub fn button_icon_style(&self) -> Option<ButtonIconStyle> {
        ButtonIconStyle::from_value(self.get_value("Windeco", "ButtonIconStyle")?)
    }

    pub fn set_button_icon_style(&mut self, style: ButtonIconStyle) {
        self.set_value("Windeco", "ButtonIconStyle", style.value().to_string());
    }

    pub fn window_corner_radius(&self) -> Option<i64> {
        self.get_int("Windeco", "WindowCornerRadius")
    }

    pub fn set_window_corner_radius(&mut self, r: i64) {
        self.set_int("Windeco", "WindowCornerRadius", r);
    }

    pub fn animations_speed_relative_system(&self) -> Option<i64> {
        self.get_int("Windeco", "AnimationsSpeedRelativeSystem")
    }

    pub fn set_animations_speed_relative_system(&mut self, v: i64) {
        self.set_int("Windeco", "AnimationsSpeedRelativeSystem", v);
    }

    pub fn blur_transparent_title_bars(&self) -> Option<bool> {
        self.get_bool("Windeco", "BlurTransparentTitleBars")
    }

    pub fn set_blur_transparent_title_bars(&mut self, v: bool) {
        self.set_bool("Windeco", "BlurTransparentTitleBars", v);
    }

    pub fn draw_title_bar_separator(&self) -> Option<bool> {
        self.get_bool("Windeco", "DrawTitleBarSeparator")
    }

    pub fn set_draw_title_bar_separator(&mut self, v: bool) {
        self.set_bool("Windeco", "DrawTitleBarSeparator", v);
    }

    pub fn match_title_bar_to_application_color(&self) -> Option<bool> {
        self.get_bool("Windeco", "MatchTitleBarToApplicationColor")
    }

    pub fn set_match_title_bar_to_application_color(&mut self, v: bool) {
        self.set_bool("Windeco", "MatchTitleBarToApplicationColor", v);
    }

    pub fn colorize_window_outline_with_button(&self) -> Option<bool> {
        self.get_bool("Windeco", "ColorizeWindowOutlineWithButton")
    }

    pub fn set_colorize_window_outline_with_button(&mut self, v: bool) {
        self.set_bool("Windeco", "ColorizeWindowOutlineWithButton", v);
    }

    pub fn window_outline_style_active(&self) -> Option<WindowOutlineStyle> {
        WindowOutlineStyle::from_value(
            self.get_value("Windeco", "WindowOutlineStyleActive")?,
        )
    }

    pub fn set_window_outline_style_active(&mut self, s: WindowOutlineStyle) {
        self.set_value("Windeco", "WindowOutlineStyleActive", s.value().to_string());
    }

    pub fn window_outline_style_inactive(&self) -> Option<WindowOutlineStyle> {
        WindowOutlineStyle::from_value(
            self.get_value("Windeco", "WindowOutlineStyleInactive")?,
        )
    }

    pub fn set_window_outline_style_inactive(&mut self, s: WindowOutlineStyle) {
        self.set_value(
            "Windeco",
            "WindowOutlineStyleInactive",
            s.value().to_string(),
        );
    }

    pub fn window_outline_custom_color_active(&self) -> Option<Rgba> {
        self.get_color("Windeco", "WindowOutlineCustomColorActive")
    }

    pub fn set_window_outline_custom_color_active(&mut self, c: Rgba) {
        self.set_color("Windeco", "WindowOutlineCustomColorActive", c);
    }

    pub fn window_outline_custom_color_inactive(&self) -> Option<Rgba> {
        self.get_color("Windeco", "WindowOutlineCustomColorInactive")
    }

    pub fn set_window_outline_custom_color_inactive(&mut self, c: Rgba) {
        self.set_color("Windeco", "WindowOutlineCustomColorInactive", c);
    }

    // -- ShadowStyle section --

    pub fn shadow_size(&self) -> Option<ShadowSize> {
        ShadowSize::from_value(self.get_value("ShadowStyle", "ShadowSize")?)
    }

    pub fn set_shadow_size(&mut self, s: ShadowSize) {
        self.set_value("ShadowStyle", "ShadowSize", s.value().to_string());
    }

    pub fn shadow_strength(&self) -> Option<i64> {
        self.get_int("ShadowStyle", "ShadowStrength")
    }

    pub fn set_shadow_strength(&mut self, v: i64) {
        self.set_int("ShadowStyle", "ShadowStrength", v);
    }

    pub fn shadow_color(&self) -> Option<Rgba> {
        self.get_color("ShadowStyle", "ShadowColor")
    }

    pub fn set_shadow_color(&mut self, c: Rgba) {
        self.set_color("ShadowStyle", "ShadowColor", c);
    }

    // -- TitleBarOpacity section --

    pub fn active_title_bar_opacity(&self) -> Option<i64> {
        self.get_int("TitleBarOpacity", "ActiveTitleBarOpacity")
    }

    pub fn set_active_title_bar_opacity(&mut self, v: i64) {
        self.set_int("TitleBarOpacity", "ActiveTitleBarOpacity", v);
    }

    pub fn inactive_title_bar_opacity(&self) -> Option<i64> {
        self.get_int("TitleBarOpacity", "InactiveTitleBarOpacity")
    }

    pub fn set_inactive_title_bar_opacity(&mut self, v: i64) {
        self.set_int("TitleBarOpacity", "InactiveTitleBarOpacity", v);
    }

    pub fn opaque_maximized_title_bars(&self) -> Option<bool> {
        self.get_bool("TitleBarOpacity", "OpaqueMaximizedTitleBars")
    }

    pub fn set_opaque_maximized_title_bars(&mut self, v: bool) {
        self.set_bool("TitleBarOpacity", "OpaqueMaximizedTitleBars", v);
    }

    // -- TitleBarSpacing section --

    pub fn title_bar_top_margin(&self) -> Option<f64> {
        self.get_f64("TitleBarSpacing", "TitleBarTopMargin")
    }

    pub fn set_title_bar_top_margin(&mut self, v: f64) {
        self.set_f64("TitleBarSpacing", "TitleBarTopMargin", v);
    }

    pub fn title_bar_bottom_margin(&self) -> Option<f64> {
        self.get_f64("TitleBarSpacing", "TitleBarBottomMargin")
    }

    pub fn set_title_bar_bottom_margin(&mut self, v: f64) {
        self.set_f64("TitleBarSpacing", "TitleBarBottomMargin", v);
    }

    pub fn percent_maximized_top_bottom_margins(&self) -> Option<i64> {
        self.get_int("TitleBarSpacing", "PercentMaximizedTopBottomMargins")
    }

    pub fn set_percent_maximized_top_bottom_margins(&mut self, v: i64) {
        self.set_int("TitleBarSpacing", "PercentMaximizedTopBottomMargins", v);
    }

    // -- ButtonColors section --

    pub fn button_background_colors_active(&self) -> Option<ButtonBackgroundColors> {
        ButtonBackgroundColors::from_value(
            self.get_value("ButtonColors", "ButtonBackgroundColorsActive")?,
        )
    }

    pub fn set_button_background_colors_active(&mut self, c: ButtonBackgroundColors) {
        self.set_value(
            "ButtonColors",
            "ButtonBackgroundColorsActive",
            c.value().to_string(),
        );
    }

    pub fn button_background_colors_inactive(&self) -> Option<ButtonBackgroundColors> {
        ButtonBackgroundColors::from_value(
            self.get_value("ButtonColors", "ButtonBackgroundColorsInactive")?,
        )
    }

    pub fn set_button_background_colors_inactive(&mut self, c: ButtonBackgroundColors) {
        self.set_value(
            "ButtonColors",
            "ButtonBackgroundColorsInactive",
            c.value().to_string(),
        );
    }

    pub fn button_background_opacity_active(&self) -> Option<i64> {
        self.get_int("ButtonColors", "ButtonBackgroundOpacityActive")
    }

    pub fn set_button_background_opacity_active(&mut self, v: i64) {
        self.set_int("ButtonColors", "ButtonBackgroundOpacityActive", v);
    }

    pub fn button_background_opacity_inactive(&self) -> Option<i64> {
        self.get_int("ButtonColors", "ButtonBackgroundOpacityInactive")
    }

    pub fn set_button_background_opacity_inactive(&mut self, v: i64) {
        self.set_int("ButtonColors", "ButtonBackgroundOpacityInactive", v);
    }

    // -- Style section --

    pub fn button_gradient(&self) -> Option<bool> {
        self.get_bool("Style", "ButtonGradient")
    }

    pub fn set_button_gradient(&mut self, v: bool) {
        self.set_bool("Style", "ButtonGradient", v);
    }

    pub fn scroll_bar_separator(&self) -> Option<bool> {
        self.get_bool("Style", "ScrollBarSeparator")
    }

    pub fn set_scroll_bar_separator(&mut self, v: bool) {
        self.set_bool("Style", "ScrollBarSeparator", v);
    }

    // ---- Serialization ---------------------------------------------------------

    /// Create a `KlassyConfig` from a parsed `IniDocument`.
    pub fn from_ini(doc: &IniDocument) -> Self {
        let mut sections = HashMap::new();
        let mut section_order = Vec::new();
        for sec in &doc.sections {
            let mut map = HashMap::new();
            for entry in &sec.entries {
                if !entry.key.is_empty() {
                    map.insert(entry.key.clone(), entry.value.clone());
                }
            }
            sections.insert(sec.name.clone(), map);
            section_order.push(sec.name.clone());
        }
        Self {
            sections,
            section_order,
        }
    }

    /// Convert this config back to an `IniDocument` for serialization.
    pub fn to_ini(&self) -> IniDocument {
        let mut doc = IniDocument {
            header_lines: Vec::new(),
            sections: Vec::new(),
            trailing_lines: Vec::new(),
        };
        for section_name in &self.section_order {
            if let Some(map) = self.sections.get(section_name) {
                let mut entries = Vec::new();
                // Sort keys for deterministic output
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
                    name: section_name.clone(),
                    entries,
                    preceding_lines: Vec::new(),
                });
            }
        }
        doc
    }
}

impl std::fmt::Display for KlassyConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count: usize = self.sections.values().map(|m| m.len()).sum();
        write!(
            f,
            "KlassyConfig({} sections, {} keys)",
            self.sections.len(),
            count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ini::parse_ini;

    #[test]
    fn from_ini_basic() {
        let input = "[Windeco]\nButtonShape=ShapeSmallCircle\nWindowCornerRadius=8\n";
        let doc = parse_ini(input);
        let config = KlassyConfig::from_ini(&doc);

        assert_eq!(config.button_shape(), Some(ButtonShape::SmallCircle));
        assert_eq!(config.window_corner_radius(), Some(8));
    }

    #[test]
    fn set_and_get() {
        let mut config = KlassyConfig::new(HashMap::new());
        config.set_button_shape(ButtonShape::FullHeightRectangle);
        assert_eq!(
            config.button_shape(),
            Some(ButtonShape::FullHeightRectangle)
        );
        assert_eq!(
            config.get_value("Windeco", "ButtonShape"),
            Some("ShapeFullHeightRectangle")
        );
    }

    #[test]
    fn typed_getters() {
        let input = concat!(
            "[Windeco]\n",
            "ButtonShape=ShapeFullHeightRectangle\n",
            "ButtonIconStyle=StyleSuessigKite\n",
            "WindowCornerRadius=8\n",
            "BlurTransparentTitleBars=true\n",
            "DrawTitleBarSeparator=false\n",
            "[ShadowStyle]\n",
            "ShadowSize=ShadowMedium\n",
            "ShadowStrength=160\n",
            "ShadowColor=255,255,255\n",
            "[TitleBarOpacity]\n",
            "ActiveTitleBarOpacity=0\n",
            "[TitleBarSpacing]\n",
            "TitleBarTopMargin=4.5\n",
            "[ButtonColors]\n",
            "ButtonBackgroundColorsActive=AccentTrafficLights\n",
            "ButtonBackgroundOpacityActive=15\n",
            "[Style]\n",
            "ButtonGradient=false\n",
            "ScrollBarSeparator=true\n",
        );
        let doc = parse_ini(input);
        let config = KlassyConfig::from_ini(&doc);

        assert_eq!(
            config.button_shape(),
            Some(ButtonShape::FullHeightRectangle)
        );
        assert_eq!(
            config.button_icon_style(),
            Some(ButtonIconStyle::SuessigKite)
        );
        assert_eq!(config.window_corner_radius(), Some(8));
        assert_eq!(config.blur_transparent_title_bars(), Some(true));
        assert_eq!(config.draw_title_bar_separator(), Some(false));
        assert_eq!(config.shadow_size(), Some(ShadowSize::Medium));
        assert_eq!(config.shadow_strength(), Some(160));
        assert_eq!(config.shadow_color(), Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(config.active_title_bar_opacity(), Some(0));
        assert_eq!(config.title_bar_top_margin(), Some(4.5));
        assert_eq!(
            config.button_background_colors_active(),
            Some(ButtonBackgroundColors::AccentTrafficLights)
        );
        assert_eq!(config.button_background_opacity_active(), Some(15));
        assert_eq!(config.button_gradient(), Some(false));
        assert_eq!(config.scroll_bar_separator(), Some(true));
    }

    #[test]
    fn round_trip_via_ini() {
        let input = "[Windeco]\nButtonShape=ShapeSmallCircle\nWindowCornerRadius=8\n";
        let doc = parse_ini(input);
        let config = KlassyConfig::from_ini(&doc);
        let doc2 = config.to_ini();

        assert_eq!(doc2.get_value("Windeco", "ButtonShape"), Some("ShapeSmallCircle"));
        assert_eq!(doc2.get_value("Windeco", "WindowCornerRadius"), Some("8"));
    }

    #[test]
    fn remove_value() {
        let mut config = KlassyConfig::new(HashMap::new());
        config.set_value("Windeco", "Test", "123".to_string());
        assert!(config.has_value("Windeco", "Test"));
        assert!(config.remove_value("Windeco", "Test"));
        assert!(!config.has_value("Windeco", "Test"));
        assert!(!config.remove_value("Windeco", "Test"));
    }

    #[test]
    fn fixture_round_trip() {
        let fixture = include_str!("../../../../../test_fixtures/klassy/sample_klassyrc");
        let doc = parse_ini(fixture);
        let config = KlassyConfig::from_ini(&doc);

        // Verify some known values from the fixture
        assert_eq!(
            config.button_background_colors_active(),
            Some(ButtonBackgroundColors::AccentTrafficLights)
        );
        assert_eq!(config.button_background_opacity_active(), Some(15));
        assert_eq!(config.shadow_size(), Some(ShadowSize::Medium));
        assert_eq!(config.shadow_strength(), Some(160));
        assert_eq!(
            config.button_shape(),
            Some(ButtonShape::FullHeightRectangle)
        );
        assert_eq!(config.window_corner_radius(), Some(8));
        assert_eq!(config.active_title_bar_opacity(), Some(0));
        assert_eq!(config.opaque_maximized_title_bars(), Some(false));
        assert_eq!(config.button_gradient(), Some(false));
        assert_eq!(config.scroll_bar_separator(), Some(true));
        assert_eq!(
            config.match_title_bar_to_application_color(),
            Some(true)
        );

        // Round-trip: config -> ini -> config should preserve values
        let doc2 = config.to_ini();
        let config2 = KlassyConfig::from_ini(&doc2);
        assert_eq!(config, config2);
    }
}
