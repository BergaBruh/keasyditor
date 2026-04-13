/// Service for applying a matugen-extracted wallpaper palette system-wide.
///
/// Rewrites Klassy, the active Kvantum theme and KDE Plasma color schemes
/// in one shot using a single source `MatugenPalette`. This OVERWRITES any
/// manual color edits the user made in those configs — callers MUST warn
/// the user before invoking this.
use std::collections::HashMap;
use std::io;

use crate::color::{self, Rgba};
use crate::constants;
use crate::services::{
    FileService, KlassyService, KvantumService, MatugenPalette, ProcessService,
};

/// Base name of the dark KDE color scheme generated from matugen.
pub const MATUGEN_DARK_SCHEME: &str = "MatugenDark";
/// Base name of the light KDE color scheme generated from matugen.
pub const MATUGEN_LIGHT_SCHEME: &str = "MatugenLight";

/// Outcome of a system-wide apply operation.
#[derive(Debug, Clone)]
pub struct WallpaperApplyOutcome {
    /// Human-readable summary of each step's result.
    pub steps: Vec<String>,
    /// Whether every step succeeded.
    pub ok: bool,
}

/// Service that applies a `MatugenPalette` to Klassy, Kvantum and KDE.
pub struct WallpaperApplyService {
    klassy: KlassyService,
    kvantum: KvantumService,
    process: ProcessService,
}

impl WallpaperApplyService {
    pub fn new() -> Self {
        Self {
            klassy: KlassyService::new(FileService::new()),
            kvantum: KvantumService::new(FileService::new()),
            process: ProcessService::new(),
        }
    }

    /// Apply the palette to Klassy, the active Kvantum theme and KDE,
    /// using `prefer_dark` to decide which matugen variant drives the engines
    /// that only support a single palette (Klassy, Kvantum, active KDE scheme).
    ///
    /// Both `MatugenDark` and `MatugenLight` KDE color schemes are always
    /// installed so the user can switch between them later.
    pub fn apply(
        &self,
        palette: &MatugenPalette,
        prefer_dark: bool,
    ) -> WallpaperApplyOutcome {
        let mut steps = Vec::new();
        let mut ok = true;

        // 1. Install both KDE color scheme files.
        match install_kde_scheme(MATUGEN_DARK_SCHEME, &palette.dark) {
            Ok(path) => steps.push(format!("Wrote {}", path)),
            Err(e) => {
                ok = false;
                steps.push(format!("Failed to write dark scheme: {}", e));
            }
        }
        match install_kde_scheme(MATUGEN_LIGHT_SCHEME, &palette.light) {
            Ok(path) => steps.push(format!("Wrote {}", path)),
            Err(e) => {
                ok = false;
                steps.push(format!("Failed to write light scheme: {}", e));
            }
        }

        let active_variant = if prefer_dark { &palette.dark } else { &palette.light };

        // 2. Rewrite Klassy colors.
        match self.apply_to_klassy(active_variant) {
            Ok(()) => steps.push("Updated Klassy colors".to_string()),
            Err(e) => {
                ok = false;
                steps.push(format!("Failed to update Klassy: {}", e));
            }
        }

        // 3. Rewrite the active Kvantum theme's [GeneralColors].
        let kvantum_theme = self.kvantum.get_active_theme();
        match &kvantum_theme {
            Some(name) => match self.apply_to_kvantum(name, active_variant) {
                Ok(()) => steps.push(format!("Updated Kvantum theme '{}'", name)),
                Err(e) => {
                    ok = false;
                    steps.push(format!("Failed to update Kvantum theme '{}': {}", name, e));
                }
            },
            None => {
                ok = false;
                steps.push("No active Kvantum theme to update".to_string());
            }
        }

        // 4. Apply KDE color scheme (matching prefer_dark).
        let scheme = if prefer_dark {
            MATUGEN_DARK_SCHEME
        } else {
            MATUGEN_LIGHT_SCHEME
        };
        match self.process.apply_plasma_colorscheme(scheme) {
            Ok(r) if r.is_success() => {
                steps.push(format!("Applied Plasma color scheme {}", scheme));
            }
            Ok(r) => {
                ok = false;
                steps.push(format!(
                    "plasma-apply-colorscheme failed: {}",
                    r.stderr.trim()
                ));
            }
            Err(e) => {
                ok = false;
                steps.push(format!("plasma-apply-colorscheme not runnable: {}", e));
            }
        }

        // 5. Re-apply Kvantum theme so the new colors take effect.
        if let Some(name) = kvantum_theme {
            match self.process.apply_kvantum_theme(&name) {
                Ok(r) if r.is_success() => {
                    steps.push(format!("Re-applied Kvantum theme '{}'", name));
                }
                Ok(r) => {
                    ok = false;
                    steps.push(format!(
                        "kvantummanager --set failed: {}",
                        r.stderr.trim()
                    ));
                }
                Err(e) => {
                    ok = false;
                    steps.push(format!("kvantummanager not runnable: {}", e));
                }
            }
        }

        // 6. Reconfigure KWin so Klassy colors take effect.
        match self.process.reconfigure_kwin() {
            Ok(_) => steps.push("Reconfigured KWin".to_string()),
            Err(e) => {
                ok = false;
                steps.push(format!("KWin reconfigure failed: {}", e));
            }
        }

        WallpaperApplyOutcome { steps, ok }
    }

    /// Load `klassyrc`, rewrite known color fields, save back.
    fn apply_to_klassy(&self, variant: &HashMap<String, String>) -> io::Result<()> {
        let mut config = self.klassy.load_config(None)?;
        for (section, key, matugen_key) in klassy_color_mapping() {
            if let Some(rgba) = lookup_color(variant, matugen_key) {
                config.set_value(section, key, color::to_klassy_color(&rgba));
            }
        }
        self.klassy.save_config(&config, None)?;
        Ok(())
    }

    /// Load the active Kvantum theme, rewrite `[GeneralColors]`, save back.
    fn apply_to_kvantum(
        &self,
        theme_name: &str,
        variant: &HashMap<String, String>,
    ) -> io::Result<()> {
        let dir_path = format!(
            "{}/{}",
            constants::kvantum_config_dir().to_string_lossy(),
            theme_name
        );
        let mut theme = self.kvantum.load_theme(&dir_path)?;

        for (key, matugen_key) in kvantum_color_mapping() {
            if let Some(rgba) = lookup_color(variant, matugen_key) {
                theme.config.colors.set_color(key, rgba);
            }
        }

        self.kvantum.save_theme(&dir_path, &theme)
    }
}

impl Default for WallpaperApplyService {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Matugen → engine mappings
// ---------------------------------------------------------------------------

/// Mapping from Klassy (`section`, `key`) to a matugen color name.
pub fn klassy_color_mapping() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("ShadowStyle", "ShadowColor", "shadow"),
        ("Windeco", "WindowOutlineCustomColorActive", "primary"),
        ("Windeco", "WindowOutlineCustomColorInactive", "outline"),
    ]
}

/// Mapping from a Kvantum `[GeneralColors]` key to a matugen color name.
pub fn kvantum_color_mapping() -> &'static [(&'static str, &'static str)] {
    &[
        ("window.color", "surface"),
        ("base.color", "background"),
        ("alt.base.color", "surface_variant"),
        ("button.color", "surface_variant"),
        ("light.color", "on_surface"),
        ("mid.light.color", "on_surface_variant"),
        ("dark.color", "outline"),
        ("mid.color", "outline"),
        ("highlight.color", "primary"),
        ("inactive.highlight.color", "primary_container"),
        ("text.color", "on_surface"),
        ("window.text.color", "on_surface"),
        ("button.text.color", "on_surface"),
        ("disabled.text.color", "on_surface_variant"),
        ("tooltip.text.color", "on_surface"),
        ("highlight.text.color", "on_primary"),
        ("link.color", "primary"),
        ("link.visited.color", "tertiary"),
        ("progress.indicator.text.color", "on_primary"),
    ]
}

/// Mapping from a KDE Plasma `[Colors:Window]` / `[Colors:Button]` / ... field
/// group to a matugen color name. Used when generating `.colors` files.
///
/// Returns `(section_name, key, matugen_key)` triples. Sections are repeated
/// across the return value because KDE expects the same fields duplicated
/// in each `[Colors:*]` group with slightly different semantics.
fn kde_scheme_entries() -> Vec<(&'static str, &'static str, &'static str)> {
    let sections = [
        "Colors:Window",
        "Colors:Button",
        "Colors:View",
        "Colors:Selection",
        "Colors:Tooltip",
        "Colors:Complementary",
        "Colors:Header",
    ];
    let mut out = Vec::new();
    for s in sections {
        // BackgroundNormal / BackgroundAlternate for container bg.
        let (bg, alt_bg) = match s {
            "Colors:Button" => ("surface_variant", "surface"),
            "Colors:Selection" => ("primary", "primary_container"),
            "Colors:Tooltip" => ("inverse_surface", "inverse_surface"),
            "Colors:Header" => ("surface_container", "surface"),
            _ => ("surface", "surface_container"),
        };
        out.push((s, "BackgroundNormal", bg));
        out.push((s, "BackgroundAlternate", alt_bg));

        // Foreground variations.
        let fg = match s {
            "Colors:Selection" => "on_primary",
            "Colors:Tooltip" => "inverse_on_surface",
            _ => "on_surface",
        };
        out.push((s, "ForegroundNormal", fg));
        out.push((s, "ForegroundActive", "primary"));
        out.push((s, "ForegroundInactive", "on_surface_variant"));
        out.push((s, "ForegroundLink", "primary"));
        out.push((s, "ForegroundVisited", "tertiary"));
        out.push((s, "ForegroundNegative", "error"));
        out.push((s, "ForegroundNeutral", "tertiary"));
        out.push((s, "ForegroundPositive", "secondary"));

        out.push((s, "DecorationFocus", "primary"));
        out.push((s, "DecorationHover", "secondary"));
    }
    out
}

// ---------------------------------------------------------------------------
// KDE .colors file writer
// ---------------------------------------------------------------------------

/// Write a KDE Plasma color scheme file for the given matugen variant into
/// `~/.local/share/color-schemes/<name>.colors`. Returns the absolute path.
pub fn install_kde_scheme(
    scheme_name: &str,
    variant: &HashMap<String, String>,
) -> io::Result<String> {
    let home = std::env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;
    let dir = format!("{}/.local/share/color-schemes", home);
    std::fs::create_dir_all(&dir)?;
    let path = format!("{}/{}.colors", dir, scheme_name);

    let content = build_kde_scheme_content(scheme_name, variant);
    std::fs::write(&path, content)?;
    Ok(path)
}

fn build_kde_scheme_content(scheme_name: &str, variant: &HashMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str("[ColorEffects:Disabled]\n");
    out.push_str("Color=56,56,56\n");
    out.push_str("ColorAmount=0\n");
    out.push_str("ColorEffect=0\n");
    out.push_str("ContrastAmount=0.65\n");
    out.push_str("ContrastEffect=1\n");
    out.push_str("IntensityAmount=0.1\n");
    out.push_str("IntensityEffect=2\n");
    out.push('\n');

    out.push_str("[ColorEffects:Inactive]\n");
    out.push_str("ChangeSelectionColor=true\n");
    out.push_str("Color=112,111,110\n");
    out.push_str("ColorAmount=0.025\n");
    out.push_str("ColorEffect=2\n");
    out.push_str("ContrastAmount=0.1\n");
    out.push_str("ContrastEffect=2\n");
    out.push_str("Enable=false\n");
    out.push_str("IntensityAmount=0\n");
    out.push_str("IntensityEffect=0\n");
    out.push('\n');

    // Group entries by section, preserving insertion order.
    let entries = kde_scheme_entries();
    let mut current_section = "";
    for (section, key, matugen_key) in entries {
        if section != current_section {
            if !current_section.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("[{}]\n", section));
            current_section = section;
        }
        let rgb = lookup_color(variant, matugen_key)
            .map(|c| format!("{},{},{}", c.r, c.g, c.b))
            .unwrap_or_else(|| "0,0,0".to_string());
        out.push_str(&format!("{}={}\n", key, rgb));
    }

    out.push_str("\n[General]\n");
    out.push_str(&format!("Name={}\n", scheme_name));
    out.push_str("shadeSortColumn=true\n");

    out.push_str("\n[KDE]\n");
    out.push_str("contrast=4\n");

    out.push_str("\n[WM]\n");
    let wm_active = lookup_color(variant, "surface_container")
        .or_else(|| lookup_color(variant, "surface"))
        .map(|c| format!("{},{},{}", c.r, c.g, c.b))
        .unwrap_or_else(|| "0,0,0".to_string());
    let wm_inactive = lookup_color(variant, "surface")
        .map(|c| format!("{},{},{}", c.r, c.g, c.b))
        .unwrap_or_else(|| "0,0,0".to_string());
    let wm_fg = lookup_color(variant, "on_surface")
        .map(|c| format!("{},{},{}", c.r, c.g, c.b))
        .unwrap_or_else(|| "255,255,255".to_string());
    out.push_str(&format!("activeBackground={}\n", wm_active));
    out.push_str(&format!("activeForeground={}\n", wm_fg));
    out.push_str(&format!("inactiveBackground={}\n", wm_inactive));
    out.push_str(&format!("inactiveForeground={}\n", wm_fg));

    out
}

/// Resolve a matugen color name to an `Rgba`. Falls back to black if missing
/// or unparseable.
fn lookup_color(variant: &HashMap<String, String>, key: &str) -> Option<Rgba> {
    variant.get(key).and_then(|v| color::try_parse(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_variant() -> HashMap<String, String> {
        let mut m = HashMap::new();
        for k in [
            "primary", "on_primary", "secondary", "tertiary", "error",
            "surface", "surface_variant", "on_surface", "on_surface_variant",
            "background", "outline", "primary_container", "surface_container",
            "inverse_surface", "inverse_on_surface", "shadow",
        ] {
            m.insert(k.to_string(), "#808080".to_string());
        }
        m
    }

    #[test]
    fn klassy_mapping_has_three_entries() {
        assert_eq!(klassy_color_mapping().len(), 3);
    }

    #[test]
    fn kvantum_mapping_covers_all_keys() {
        use crate::models::kvantum::KvantumColors;
        let mapped: std::collections::HashSet<&str> =
            kvantum_color_mapping().iter().map(|(k, _)| *k).collect();
        for k in KvantumColors::all_keys() {
            assert!(
                mapped.contains(k),
                "Kvantum color key '{}' is not mapped",
                k
            );
        }
    }

    #[test]
    fn build_kde_scheme_contains_all_sections() {
        let content = build_kde_scheme_content("MatugenTest", &fake_variant());
        assert!(content.contains("[Colors:Window]"));
        assert!(content.contains("[Colors:Button]"));
        assert!(content.contains("[Colors:View]"));
        assert!(content.contains("[Colors:Selection]"));
        assert!(content.contains("[Colors:Tooltip]"));
        assert!(content.contains("[Colors:Complementary]"));
        assert!(content.contains("[Colors:Header]"));
        assert!(content.contains("[General]"));
        assert!(content.contains("Name=MatugenTest"));
        assert!(content.contains("[WM]"));
        assert!(content.contains("BackgroundNormal=128,128,128"));
    }

    #[test]
    fn build_kde_scheme_missing_keys_fall_back_to_black() {
        let content = build_kde_scheme_content("Empty", &HashMap::new());
        assert!(content.contains("BackgroundNormal=0,0,0"));
        assert!(content.contains("Name=Empty"));
    }
}
