/// Data model for the `[GeneralColors]` section of a Kvantum theme config.
///
/// Provides typed `Rgba` accessors for all 19 known color keys, plus
/// generic string-based access for any custom keys.
use std::collections::HashMap;

use crate::color::{self, Rgba};

/// All known color keys in the `[GeneralColors]` section.
pub static ALL_COLOR_KEYS: &[&str] = &[
    "window.color",
    "base.color",
    "alt.base.color",
    "button.color",
    "light.color",
    "mid.light.color",
    "dark.color",
    "mid.color",
    "highlight.color",
    "inactive.highlight.color",
    "text.color",
    "window.text.color",
    "button.text.color",
    "disabled.text.color",
    "tooltip.text.color",
    "highlight.text.color",
    "link.color",
    "link.visited.color",
    "progress.indicator.text.color",
];

/// Model for the `[GeneralColors]` section of a Kvantum `.kvconfig` file.
#[derive(Clone, Debug, PartialEq)]
pub struct KvantumColors {
    values: HashMap<String, String>,
}

impl KvantumColors {
    /// Create from a raw key-value map.
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self { values: map }
    }

    /// Create an empty instance.
    pub fn empty() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// All known color key names.
    pub fn all_keys() -> &'static [&'static str] {
        ALL_COLOR_KEYS
    }

    // --------------------------------------------------------------------------
    // Generic access
    // --------------------------------------------------------------------------

    /// Get the raw string value for a key, or `None` if absent.
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Set a raw string value.
    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    /// Parse a color value for a key.
    pub fn get_color(&self, key: &str) -> Option<Rgba> {
        color::try_parse(self.values.get(key)?)
    }

    /// Set a color for a key using Kvantum hex notation (`#rrggbb`).
    pub fn set_color(&mut self, key: &str, c: Rgba) {
        self.values
            .insert(key.to_string(), color::to_kvantum_color(&c));
    }

    /// Return a copy of the underlying map.
    pub fn to_map(&self) -> HashMap<String, String> {
        self.values.clone()
    }

    /// Number of key-value pairs.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the colors map is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // --------------------------------------------------------------------------
    // Typed color accessors
    // --------------------------------------------------------------------------

    fn color_or_black(&self, key: &str) -> Rgba {
        self.get_color(key).unwrap_or(Rgba::rgb(0, 0, 0))
    }

    pub fn window_color(&self) -> Rgba {
        self.color_or_black("window.color")
    }
    pub fn set_window_color(&mut self, c: Rgba) {
        self.set_color("window.color", c);
    }

    pub fn base_color(&self) -> Rgba {
        self.color_or_black("base.color")
    }
    pub fn set_base_color(&mut self, c: Rgba) {
        self.set_color("base.color", c);
    }

    pub fn alt_base_color(&self) -> Rgba {
        self.color_or_black("alt.base.color")
    }
    pub fn set_alt_base_color(&mut self, c: Rgba) {
        self.set_color("alt.base.color", c);
    }

    pub fn button_color(&self) -> Rgba {
        self.color_or_black("button.color")
    }
    pub fn set_button_color(&mut self, c: Rgba) {
        self.set_color("button.color", c);
    }

    pub fn light_color(&self) -> Rgba {
        self.color_or_black("light.color")
    }
    pub fn set_light_color(&mut self, c: Rgba) {
        self.set_color("light.color", c);
    }

    pub fn mid_light_color(&self) -> Rgba {
        self.color_or_black("mid.light.color")
    }
    pub fn set_mid_light_color(&mut self, c: Rgba) {
        self.set_color("mid.light.color", c);
    }

    pub fn dark_color(&self) -> Rgba {
        self.color_or_black("dark.color")
    }
    pub fn set_dark_color(&mut self, c: Rgba) {
        self.set_color("dark.color", c);
    }

    pub fn mid_color(&self) -> Rgba {
        self.color_or_black("mid.color")
    }
    pub fn set_mid_color(&mut self, c: Rgba) {
        self.set_color("mid.color", c);
    }

    pub fn highlight_color(&self) -> Rgba {
        self.color_or_black("highlight.color")
    }
    pub fn set_highlight_color(&mut self, c: Rgba) {
        self.set_color("highlight.color", c);
    }

    pub fn inactive_highlight_color(&self) -> Rgba {
        self.color_or_black("inactive.highlight.color")
    }
    pub fn set_inactive_highlight_color(&mut self, c: Rgba) {
        self.set_color("inactive.highlight.color", c);
    }

    pub fn text_color(&self) -> Rgba {
        self.color_or_black("text.color")
    }
    pub fn set_text_color(&mut self, c: Rgba) {
        self.set_color("text.color", c);
    }

    pub fn window_text_color(&self) -> Rgba {
        self.color_or_black("window.text.color")
    }
    pub fn set_window_text_color(&mut self, c: Rgba) {
        self.set_color("window.text.color", c);
    }

    pub fn button_text_color(&self) -> Rgba {
        self.color_or_black("button.text.color")
    }
    pub fn set_button_text_color(&mut self, c: Rgba) {
        self.set_color("button.text.color", c);
    }

    pub fn disabled_text_color(&self) -> Rgba {
        self.color_or_black("disabled.text.color")
    }
    pub fn set_disabled_text_color(&mut self, c: Rgba) {
        self.set_color("disabled.text.color", c);
    }

    pub fn tooltip_text_color(&self) -> Rgba {
        self.color_or_black("tooltip.text.color")
    }
    pub fn set_tooltip_text_color(&mut self, c: Rgba) {
        self.set_color("tooltip.text.color", c);
    }

    pub fn highlight_text_color(&self) -> Rgba {
        self.color_or_black("highlight.text.color")
    }
    pub fn set_highlight_text_color(&mut self, c: Rgba) {
        self.set_color("highlight.text.color", c);
    }

    pub fn link_color(&self) -> Rgba {
        self.color_or_black("link.color")
    }
    pub fn set_link_color(&mut self, c: Rgba) {
        self.set_color("link.color", c);
    }

    pub fn link_visited_color(&self) -> Rgba {
        self.color_or_black("link.visited.color")
    }
    pub fn set_link_visited_color(&mut self, c: Rgba) {
        self.set_color("link.visited.color", c);
    }

    pub fn progress_indicator_text_color(&self) -> Rgba {
        self.color_or_black("progress.indicator.text.color")
    }
    pub fn set_progress_indicator_text_color(&mut self, c: Rgba) {
        self.set_color("progress.indicator.text.color", c);
    }
}

impl std::fmt::Display for KvantumColors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KvantumColors({} keys)", self.values.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_map_and_accessors() {
        let mut map = HashMap::new();
        map.insert("window.color".to_string(), "#1a1111".to_string());
        map.insert("text.color".to_string(), "#f1dedd".to_string());
        map.insert(
            "progress.indicator.text.color".to_string(),
            "white".to_string(),
        );

        let colors = KvantumColors::from_map(map);
        assert_eq!(colors.window_color(), Rgba::rgb(0x1a, 0x11, 0x11));
        assert_eq!(colors.text_color(), Rgba::rgb(0xf1, 0xde, 0xdd));
        assert_eq!(
            colors.progress_indicator_text_color(),
            Rgba::rgb(255, 255, 255)
        );
    }

    #[test]
    fn set_color_uses_kvantum_format() {
        let mut colors = KvantumColors::empty();
        colors.set_window_color(Rgba::rgb(0x1a, 0x11, 0x11));
        assert_eq!(colors.get_value("window.color"), Some("#1a1111"));
    }

    #[test]
    fn missing_color_returns_black() {
        let colors = KvantumColors::empty();
        assert_eq!(colors.window_color(), Rgba::rgb(0, 0, 0));
    }

    #[test]
    fn all_keys_count() {
        assert_eq!(KvantumColors::all_keys().len(), 19);
    }
}
