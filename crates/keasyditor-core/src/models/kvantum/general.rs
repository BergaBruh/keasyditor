/// Data model for the `[%General]` section of a Kvantum theme config.
///
/// Provides typed accessors for the ~64 known keys as well as generic
/// string-based get/set for any key.
use std::collections::HashMap;

/// Model for the `[%General]` section of a Kvantum `.kvconfig` file.
#[derive(Clone, Debug, PartialEq)]
pub struct KvantumGeneral {
    values: HashMap<String, String>,
}

impl KvantumGeneral {
    /// Create from a raw key-value map.
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self { values: map }
    }

    /// Create an empty instance (all defaults).
    pub fn empty() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    // --------------------------------------------------------------------------
    // Generic access
    // --------------------------------------------------------------------------

    /// Get a raw string value by key, or `None` if not present.
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Set a raw string value by key.
    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    /// Remove a key entirely.
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

    /// Whether the general section has no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // --------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------

    fn string(&self, key: &str, fallback: &str) -> String {
        self.values
            .get(key)
            .cloned()
            .unwrap_or_else(|| fallback.to_string())
    }

    fn bool_val(&self, key: &str, fallback: bool) -> bool {
        match self.values.get(key) {
            Some(v) => v == "true",
            None => fallback,
        }
    }

    fn int_val(&self, key: &str, fallback: i64) -> i64 {
        self.values
            .get(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(fallback)
    }

    fn f64_val(&self, key: &str, fallback: f64) -> f64 {
        self.values
            .get(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(fallback)
    }

    fn set_string(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    fn set_bool_val(&mut self, key: &str, value: bool) {
        self.values.insert(key.to_string(), value.to_string());
    }

    fn set_int_val(&mut self, key: &str, value: i64) {
        self.values.insert(key.to_string(), value.to_string());
    }

    fn set_f64_val(&mut self, key: &str, value: f64) {
        self.values
            .insert(key.to_string(), format!("{:.2}", value));
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- metadata
    // --------------------------------------------------------------------------

    pub fn author(&self) -> String {
        self.string("author", "")
    }
    pub fn set_author(&mut self, v: &str) {
        self.set_string("author", v);
    }

    pub fn comment(&self) -> String {
        self.string("comment", "")
    }
    pub fn set_comment(&mut self, v: &str) {
        self.set_string("comment", v);
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- booleans
    // --------------------------------------------------------------------------

    pub fn left_tabs(&self) -> bool {
        self.bool_val("left_tabs", false)
    }
    pub fn set_left_tabs(&mut self, v: bool) {
        self.set_bool_val("left_tabs", v);
    }

    pub fn attach_active_tab(&self) -> bool {
        self.bool_val("attach_active_tab", false)
    }
    pub fn set_attach_active_tab(&mut self, v: bool) {
        self.set_bool_val("attach_active_tab", v);
    }

    pub fn no_window_pattern(&self) -> bool {
        self.bool_val("no_window_pattern", false)
    }
    pub fn set_no_window_pattern(&mut self, v: bool) {
        self.set_bool_val("no_window_pattern", v);
    }

    pub fn group_toolbar_buttons(&self) -> bool {
        self.bool_val("group_toolbar_buttons", false)
    }
    pub fn set_group_toolbar_buttons(&mut self, v: bool) {
        self.set_bool_val("group_toolbar_buttons", v);
    }

    pub fn spread_progressbar(&self) -> bool {
        self.bool_val("spread_progressbar", false)
    }
    pub fn set_spread_progressbar(&mut self, v: bool) {
        self.set_bool_val("spread_progressbar", v);
    }

    pub fn composite(&self) -> bool {
        self.bool_val("composite", true)
    }
    pub fn set_composite(&mut self, v: bool) {
        self.set_bool_val("composite", v);
    }

    pub fn spread_menuitems(&self) -> bool {
        self.bool_val("spread_menuitems", false)
    }
    pub fn set_spread_menuitems(&mut self, v: bool) {
        self.set_bool_val("spread_menuitems", v);
    }

    pub fn popup_blurring(&self) -> bool {
        self.bool_val("popup_blurring", false)
    }
    pub fn set_popup_blurring(&mut self, v: bool) {
        self.set_bool_val("popup_blurring", v);
    }

    pub fn menubar_mouse_tracking(&self) -> bool {
        self.bool_val("menubar_mouse_tracking", true)
    }
    pub fn set_menubar_mouse_tracking(&mut self, v: bool) {
        self.set_bool_val("menubar_mouse_tracking", v);
    }

    pub fn vertical_spin_buttons(&self) -> bool {
        self.bool_val("vertical_spin_buttons", true)
    }
    pub fn set_vertical_spin_buttons(&mut self, v: bool) {
        self.set_bool_val("vertical_spin_buttons", v);
    }

    pub fn translucent_windows(&self) -> bool {
        self.bool_val("translucent_windows", false)
    }
    pub fn set_translucent_windows(&mut self, v: bool) {
        self.set_bool_val("translucent_windows", v);
    }

    pub fn blurring(&self) -> bool {
        self.bool_val("blurring", false)
    }
    pub fn set_blurring(&mut self, v: bool) {
        self.set_bool_val("blurring", v);
    }

    pub fn animate_states(&self) -> bool {
        self.bool_val("animate_states", false)
    }
    pub fn set_animate_states(&mut self, v: bool) {
        self.set_bool_val("animate_states", v);
    }

    pub fn combo_as_lineedit(&self) -> bool {
        self.bool_val("combo_as_lineedit", false)
    }
    pub fn set_combo_as_lineedit(&mut self, v: bool) {
        self.set_bool_val("combo_as_lineedit", v);
    }

    pub fn combo_menu(&self) -> bool {
        self.bool_val("combo_menu", false)
    }
    pub fn set_combo_menu(&mut self, v: bool) {
        self.set_bool_val("combo_menu", v);
    }

    pub fn scroll_arrows(&self) -> bool {
        self.bool_val("scroll_arrows", true)
    }
    pub fn set_scroll_arrows(&mut self, v: bool) {
        self.set_bool_val("scroll_arrows", v);
    }

    pub fn fill_rubberband(&self) -> bool {
        self.bool_val("fill_rubberband", false)
    }
    pub fn set_fill_rubberband(&mut self, v: bool) {
        self.set_bool_val("fill_rubberband", v);
    }

    pub fn transient_scrollbar(&self) -> bool {
        self.bool_val("transient_scrollbar", false)
    }
    pub fn set_transient_scrollbar(&mut self, v: bool) {
        self.set_bool_val("transient_scrollbar", v);
    }

    pub fn alt_mnemonic(&self) -> bool {
        self.bool_val("alt_mnemonic", true)
    }
    pub fn set_alt_mnemonic(&mut self, v: bool) {
        self.set_bool_val("alt_mnemonic", v);
    }

    pub fn respect_de(&self) -> bool {
        self.bool_val("respect_DE", true)
    }
    pub fn set_respect_de(&mut self, v: bool) {
        self.set_bool_val("respect_DE", v);
    }

    pub fn scrollable_menu(&self) -> bool {
        self.bool_val("scrollable_menu", true)
    }
    pub fn set_scrollable_menu(&mut self, v: bool) {
        self.set_bool_val("scrollable_menu", v);
    }

    pub fn tree_branch_line(&self) -> bool {
        self.bool_val("tree_branch_line", false)
    }
    pub fn set_tree_branch_line(&mut self, v: bool) {
        self.set_bool_val("tree_branch_line", v);
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- integers
    // --------------------------------------------------------------------------

    pub fn progressbar_thickness(&self) -> i64 {
        self.int_val("progressbar_thickness", 6)
    }
    pub fn set_progressbar_thickness(&mut self, v: i64) {
        self.set_int_val("progressbar_thickness", v);
    }

    pub fn menu_shadow_depth(&self) -> i64 {
        self.int_val("menu_shadow_depth", 6)
    }
    pub fn set_menu_shadow_depth(&mut self, v: i64) {
        self.set_int_val("menu_shadow_depth", v);
    }

    pub fn menu_blur_radius(&self) -> i64 {
        self.int_val("menu_blur_radius", 0)
    }
    pub fn set_menu_blur_radius(&mut self, v: i64) {
        self.set_int_val("menu_blur_radius", v);
    }

    pub fn tooltip_shadow_depth(&self) -> i64 {
        self.int_val("tooltip_shadow_depth", 6)
    }
    pub fn set_tooltip_shadow_depth(&mut self, v: i64) {
        self.set_int_val("tooltip_shadow_depth", v);
    }

    pub fn slider_width(&self) -> i64 {
        self.int_val("slider_width", 6)
    }
    pub fn set_slider_width(&mut self, v: i64) {
        self.set_int_val("slider_width", v);
    }

    pub fn slider_handle_width(&self) -> i64 {
        self.int_val("slider_handle_width", 16)
    }
    pub fn set_slider_handle_width(&mut self, v: i64) {
        self.set_int_val("slider_handle_width", v);
    }

    pub fn slider_handle_length(&self) -> i64 {
        self.int_val("slider_handle_length", 16)
    }
    pub fn set_slider_handle_length(&mut self, v: i64) {
        self.set_int_val("slider_handle_length", v);
    }

    pub fn scroll_min_extent(&self) -> i64 {
        self.int_val("scroll_min_extent", 36)
    }
    pub fn set_scroll_min_extent(&mut self, v: i64) {
        self.set_int_val("scroll_min_extent", v);
    }

    pub fn toolbutton_style(&self) -> i64 {
        self.int_val("toolbutton_style", 0)
    }
    pub fn set_toolbutton_style(&mut self, v: i64) {
        self.set_int_val("toolbutton_style", v);
    }

    pub fn click_behavior(&self) -> i64 {
        self.int_val("click_behavior", 0)
    }
    pub fn set_click_behavior(&mut self, v: i64) {
        self.set_int_val("click_behavior", v);
    }

    pub fn small_icon_size(&self) -> i64 {
        self.int_val("small_icon_size", 16)
    }
    pub fn set_small_icon_size(&mut self, v: i64) {
        self.set_int_val("small_icon_size", v);
    }

    pub fn large_icon_size(&self) -> i64 {
        self.int_val("large_icon_size", 32)
    }
    pub fn set_large_icon_size(&mut self, v: i64) {
        self.set_int_val("large_icon_size", v);
    }

    pub fn layout_spacing(&self) -> i64 {
        self.int_val("layout_spacing", 2)
    }
    pub fn set_layout_spacing(&mut self, v: i64) {
        self.set_int_val("layout_spacing", v);
    }

    pub fn layout_margin(&self) -> i64 {
        self.int_val("layout_margin", 4)
    }
    pub fn set_layout_margin(&mut self, v: i64) {
        self.set_int_val("layout_margin", v);
    }

    pub fn reduce_window_opacity(&self) -> i64 {
        self.int_val("reduce_window_opacity", 0)
    }
    pub fn set_reduce_window_opacity(&mut self, v: i64) {
        self.set_int_val("reduce_window_opacity", v);
    }

    pub fn reduce_menu_opacity(&self) -> i64 {
        self.int_val("reduce_menu_opacity", 0)
    }
    pub fn set_reduce_menu_opacity(&mut self, v: i64) {
        self.set_int_val("reduce_menu_opacity", v);
    }

    pub fn submenu_delay(&self) -> i64 {
        self.int_val("submenu_delay", 250)
    }
    pub fn set_submenu_delay(&mut self, v: i64) {
        self.set_int_val("submenu_delay", v);
    }

    pub fn tooltip_delay(&self) -> i64 {
        self.int_val("tooltip_delay", -1)
    }
    pub fn set_tooltip_delay(&mut self, v: i64) {
        self.set_int_val("tooltip_delay", v);
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- doubles
    // --------------------------------------------------------------------------

    pub fn contrast(&self) -> f64 {
        self.f64_val("contrast", 1.0)
    }
    pub fn set_contrast(&mut self, v: f64) {
        self.set_f64_val("contrast", v);
    }

    pub fn intensity(&self) -> f64 {
        self.f64_val("intensity", 1.0)
    }
    pub fn set_intensity(&mut self, v: f64) {
        self.set_f64_val("intensity", v);
    }

    pub fn saturation(&self) -> f64 {
        self.f64_val("saturation", 1.0)
    }
    pub fn set_saturation(&mut self, v: f64) {
        self.set_f64_val("saturation", v);
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- strings / enums
    // --------------------------------------------------------------------------

    /// X11 drag mode: `"all"`, `"menubar"`, or `"none"`.
    pub fn x11drag(&self) -> String {
        self.string("x11drag", "all")
    }
    pub fn set_x11drag(&mut self, v: &str) {
        self.set_string("x11drag", v);
    }

    // --------------------------------------------------------------------------
    // Typed accessors -- lists
    // --------------------------------------------------------------------------

    /// Comma-separated list of apps that should remain opaque.
    pub fn opaque_apps(&self) -> Vec<String> {
        match self.values.get("opaque") {
            Some(raw) if !raw.is_empty() => {
                raw.split(',').map(|s| s.trim().to_string()).collect()
            }
            _ => Vec::new(),
        }
    }

    pub fn set_opaque_apps(&mut self, apps: &[String]) {
        self.values
            .insert("opaque".to_string(), apps.join(","));
    }
}

impl std::fmt::Display for KvantumGeneral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KvantumGeneral({} keys)", self.values.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let g = KvantumGeneral::empty();
        assert!(g.composite());
        assert_eq!(g.progressbar_thickness(), 6);
        assert!((g.contrast() - 1.0).abs() < f64::EPSILON);
        assert_eq!(g.x11drag(), "all");
        assert!(g.opaque_apps().is_empty());
    }

    #[test]
    fn from_map() {
        let mut map = HashMap::new();
        map.insert("author".to_string(), "Custom".to_string());
        map.insert("composite".to_string(), "true".to_string());
        map.insert("progressbar_thickness".to_string(), "8".to_string());
        map.insert("contrast".to_string(), "1.00".to_string());

        let g = KvantumGeneral::from_map(map);
        assert_eq!(g.author(), "Custom");
        assert!(g.composite());
        assert_eq!(g.progressbar_thickness(), 8);
        assert!((g.contrast() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn set_and_get() {
        let mut g = KvantumGeneral::empty();
        g.set_author("Test");
        assert_eq!(g.author(), "Test");
        g.set_composite(false);
        assert!(!g.composite());
        g.set_progressbar_thickness(10);
        assert_eq!(g.progressbar_thickness(), 10);
    }

    #[test]
    fn opaque_apps() {
        let mut g = KvantumGeneral::empty();
        g.set_opaque_apps(&["vlc".to_string(), "kaffeine".to_string()]);
        assert_eq!(g.opaque_apps(), vec!["vlc", "kaffeine"]);
    }
}
