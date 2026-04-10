/// Data model for a generic widget section in a Kvantum theme config.
///
/// Widget sections (e.g. `[PanelButtonCommand]`, `[Tab]`, `[ComboBox]`) share
/// a common set of properties and support single-parent inheritance via the
/// `inherits` key.
use std::collections::HashMap;

/// Model for a widget section in a Kvantum `.kvconfig` file.
#[derive(Clone, Debug, PartialEq)]
pub struct KvantumSection {
    /// The section name as it appears in the INI file.
    pub name: String,
    /// Underlying key-value pairs (includes the `inherits` key if present).
    values: HashMap<String, String>,
}

impl KvantumSection {
    /// Create from a section name and its raw key-value map.
    pub fn from_map(name: &str, map: HashMap<String, String>) -> Self {
        Self {
            name: name.to_string(),
            values: map,
        }
    }

    /// Create an empty section.
    pub fn empty(name: &str) -> Self {
        Self {
            name: name.to_string(),
            values: HashMap::new(),
        }
    }

    // --------------------------------------------------------------------------
    // Inheritance
    // --------------------------------------------------------------------------

    /// The name of the parent section this section inherits from, or `None`.
    pub fn inherits(&self) -> Option<&str> {
        self.values.get("inherits").map(|s| s.as_str())
    }

    /// Whether this section inherits from another.
    pub fn has_inheritance(&self) -> bool {
        self.inherits().is_some()
    }

    /// Create a new `KvantumSection` that merges `parent`'s values with this
    /// section's overrides.
    ///
    /// The returned section has no `inherits` key since it is fully resolved.
    /// Properties in *this* section take precedence over parent.
    pub fn resolve_with(&self, parent: &KvantumSection) -> Self {
        let mut merged = parent.values.clone();
        // Remove parent's inherits -- we are flattening.
        merged.remove("inherits");
        // Apply our overrides on top.
        for (k, v) in &self.values {
            merged.insert(k.clone(), v.clone());
        }
        // Remove the inherits key from the merged result.
        merged.remove("inherits");
        Self {
            name: self.name.clone(),
            values: merged,
        }
    }

    // --------------------------------------------------------------------------
    // Generic access
    // --------------------------------------------------------------------------

    /// Get a raw string value by key, or `None` if absent.
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Set a raw string value.
    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    /// Remove a key.
    pub fn remove_key(&mut self, key: &str) {
        self.values.remove(key);
    }

    /// Return a copy of the underlying map.
    pub fn to_map(&self) -> HashMap<String, String> {
        self.values.clone()
    }

    /// Number of key-value pairs.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the section has no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // --------------------------------------------------------------------------
    // Typed helpers
    // --------------------------------------------------------------------------

    fn get_bool(&self, key: &str) -> Option<bool> {
        Some(self.values.get(key)? == "true")
    }

    fn get_int(&self, key: &str) -> Option<i64> {
        self.values.get(key)?.parse().ok()
    }

    fn set_bool(&mut self, key: &str, value: Option<bool>) {
        match value {
            Some(v) => {
                self.values.insert(key.to_string(), v.to_string());
            }
            None => {
                self.values.remove(key);
            }
        }
    }

    fn set_int(&mut self, key: &str, value: Option<i64>) {
        match value {
            Some(v) => {
                self.values.insert(key.to_string(), v.to_string());
            }
            None => {
                self.values.remove(key);
            }
        }
    }

    // --------------------------------------------------------------------------
    // Frame properties
    // --------------------------------------------------------------------------

    pub fn frame(&self) -> Option<bool> {
        self.get_bool("frame")
    }
    pub fn set_frame(&mut self, v: Option<bool>) {
        self.set_bool("frame", v);
    }

    pub fn frame_element(&self) -> Option<&str> {
        self.get_value("frame.element")
    }
    pub fn set_frame_element(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("frame.element", s),
            None => self.remove_key("frame.element"),
        }
    }

    pub fn frame_top(&self) -> Option<i64> {
        self.get_int("frame.top")
    }
    pub fn set_frame_top(&mut self, v: Option<i64>) {
        self.set_int("frame.top", v);
    }

    pub fn frame_bottom(&self) -> Option<i64> {
        self.get_int("frame.bottom")
    }
    pub fn set_frame_bottom(&mut self, v: Option<i64>) {
        self.set_int("frame.bottom", v);
    }

    pub fn frame_left(&self) -> Option<i64> {
        self.get_int("frame.left")
    }
    pub fn set_frame_left(&mut self, v: Option<i64>) {
        self.set_int("frame.left", v);
    }

    pub fn frame_right(&self) -> Option<i64> {
        self.get_int("frame.right")
    }
    pub fn set_frame_right(&mut self, v: Option<i64>) {
        self.set_int("frame.right", v);
    }

    pub fn frame_expansion(&self) -> Option<i64> {
        self.get_int("frame.expansion")
    }
    pub fn set_frame_expansion(&mut self, v: Option<i64>) {
        self.set_int("frame.expansion", v);
    }

    // --------------------------------------------------------------------------
    // Interior properties
    // --------------------------------------------------------------------------

    pub fn interior(&self) -> Option<bool> {
        self.get_bool("interior")
    }
    pub fn set_interior(&mut self, v: Option<bool>) {
        self.set_bool("interior", v);
    }

    pub fn interior_element(&self) -> Option<&str> {
        self.get_value("interior.element")
    }
    pub fn set_interior_element(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("interior.element", s),
            None => self.remove_key("interior.element"),
        }
    }

    // --------------------------------------------------------------------------
    // Text properties
    // --------------------------------------------------------------------------

    pub fn text_normal_color(&self) -> Option<&str> {
        self.get_value("text.normal.color")
    }
    pub fn set_text_normal_color(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("text.normal.color", s),
            None => self.remove_key("text.normal.color"),
        }
    }

    pub fn text_focus_color(&self) -> Option<&str> {
        self.get_value("text.focus.color")
    }
    pub fn set_text_focus_color(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("text.focus.color", s),
            None => self.remove_key("text.focus.color"),
        }
    }

    pub fn text_press_color(&self) -> Option<&str> {
        self.get_value("text.press.color")
    }
    pub fn set_text_press_color(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("text.press.color", s),
            None => self.remove_key("text.press.color"),
        }
    }

    pub fn text_toggle_color(&self) -> Option<&str> {
        self.get_value("text.toggle.color")
    }
    pub fn set_text_toggle_color(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("text.toggle.color", s),
            None => self.remove_key("text.toggle.color"),
        }
    }

    pub fn text_bold(&self) -> Option<bool> {
        self.get_bool("text.bold")
    }
    pub fn set_text_bold(&mut self, v: Option<bool>) {
        self.set_bool("text.bold", v);
    }

    pub fn text_italic(&self) -> Option<bool> {
        self.get_bool("text.italic")
    }
    pub fn set_text_italic(&mut self, v: Option<bool>) {
        self.set_bool("text.italic", v);
    }

    pub fn text_margin(&self) -> Option<&str> {
        self.get_value("text.margin")
    }
    pub fn set_text_margin(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("text.margin", s),
            None => self.remove_key("text.margin"),
        }
    }

    pub fn text_margin_top(&self) -> Option<i64> {
        self.get_int("text.margin.top")
    }
    pub fn set_text_margin_top(&mut self, v: Option<i64>) {
        self.set_int("text.margin.top", v);
    }

    pub fn text_margin_bottom(&self) -> Option<i64> {
        self.get_int("text.margin.bottom")
    }
    pub fn set_text_margin_bottom(&mut self, v: Option<i64>) {
        self.set_int("text.margin.bottom", v);
    }

    pub fn text_margin_left(&self) -> Option<i64> {
        self.get_int("text.margin.left")
    }
    pub fn set_text_margin_left(&mut self, v: Option<i64>) {
        self.set_int("text.margin.left", v);
    }

    pub fn text_margin_right(&self) -> Option<i64> {
        self.get_int("text.margin.right")
    }
    pub fn set_text_margin_right(&mut self, v: Option<i64>) {
        self.set_int("text.margin.right", v);
    }

    // --------------------------------------------------------------------------
    // Indicator properties
    // --------------------------------------------------------------------------

    pub fn indicator_element(&self) -> Option<&str> {
        self.get_value("indicator.element")
    }
    pub fn set_indicator_element(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("indicator.element", s),
            None => self.remove_key("indicator.element"),
        }
    }

    pub fn indicator_size(&self) -> Option<i64> {
        self.get_int("indicator.size")
    }
    pub fn set_indicator_size(&mut self, v: Option<i64>) {
        self.set_int("indicator.size", v);
    }

    // --------------------------------------------------------------------------
    // Size properties
    // --------------------------------------------------------------------------

    pub fn min_width(&self) -> Option<&str> {
        self.get_value("min_width")
    }
    pub fn set_min_width(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("min_width", s),
            None => self.remove_key("min_width"),
        }
    }

    pub fn min_height(&self) -> Option<&str> {
        self.get_value("min_height")
    }
    pub fn set_min_height(&mut self, v: Option<String>) {
        match v {
            Some(s) => self.set_value("min_height", s),
            None => self.remove_key("min_height"),
        }
    }

    // --------------------------------------------------------------------------
    // Focus
    // --------------------------------------------------------------------------

    pub fn focus_frame(&self) -> Option<bool> {
        self.get_bool("focusFrame")
    }
    pub fn set_focus_frame(&mut self, v: Option<bool>) {
        self.set_bool("focusFrame", v);
    }
}

impl std::fmt::Display for KvantumSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KvantumSection({}{}), {} keys)",
            self.name,
            if let Some(parent) = self.inherits() {
                format!(" inherits={}", parent)
            } else {
                String::new()
            },
            self.values.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_map_basic() {
        let mut map = HashMap::new();
        map.insert("frame".to_string(), "true".to_string());
        map.insert("frame.top".to_string(), "3".to_string());
        map.insert("text.normal.color".to_string(), "#cccccc".to_string());

        let section = KvantumSection::from_map("PanelButtonCommand", map);
        assert_eq!(section.frame(), Some(true));
        assert_eq!(section.frame_top(), Some(3));
        assert_eq!(section.text_normal_color(), Some("#cccccc"));
        assert!(!section.has_inheritance());
    }

    #[test]
    fn inheritance() {
        let mut parent_map = HashMap::new();
        parent_map.insert("frame".to_string(), "true".to_string());
        parent_map.insert("frame.top".to_string(), "3".to_string());
        parent_map.insert("text.normal.color".to_string(), "#aaaaaa".to_string());
        let parent = KvantumSection::from_map("PanelButtonCommand", parent_map);

        let mut child_map = HashMap::new();
        child_map.insert("inherits".to_string(), "PanelButtonCommand".to_string());
        child_map.insert("text.normal.color".to_string(), "#ffffff".to_string());
        let child = KvantumSection::from_map("ToolbarButton", child_map);

        assert!(child.has_inheritance());
        assert_eq!(child.inherits(), Some("PanelButtonCommand"));

        let resolved = child.resolve_with(&parent);
        assert_eq!(resolved.frame(), Some(true)); // from parent
        assert_eq!(resolved.frame_top(), Some(3)); // from parent
        assert_eq!(resolved.text_normal_color(), Some("#ffffff")); // overridden
        assert!(!resolved.has_inheritance()); // inherits key removed
    }

    #[test]
    fn empty_section() {
        let section = KvantumSection::empty("Test");
        assert_eq!(section.name, "Test");
        assert!(section.is_empty());
        assert_eq!(section.frame(), None);
    }

    #[test]
    fn set_and_remove() {
        let mut section = KvantumSection::empty("Test");
        section.set_frame(Some(true));
        assert_eq!(section.frame(), Some(true));
        section.set_frame(None);
        assert_eq!(section.frame(), None);
    }
}
