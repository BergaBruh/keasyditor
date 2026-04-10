/// Service for extracting Material You color palettes using matugen.
use std::collections::HashMap;
use std::process::Command;

/// A Material You color palette extracted by matugen.
#[derive(Clone, Debug, PartialEq)]
pub struct MatugenPalette {
    /// Path to the source image.
    pub image_path: String,
    /// Dark-mode colors: Material You color name -> hex string (e.g. `#1a1110`).
    pub dark: HashMap<String, String>,
    /// Light-mode colors.
    pub light: HashMap<String, String>,
}

impl MatugenPalette {
    /// The subset of colors most useful for theming.
    pub const KEY_COLORS: &'static [&'static str] = &[
        "source_color",
        "primary",
        "on_primary",
        "primary_container",
        "secondary",
        "on_secondary",
        "secondary_container",
        "tertiary",
        "on_tertiary",
        "tertiary_container",
        "surface",
        "on_surface",
        "surface_variant",
        "on_surface_variant",
        "background",
        "on_background",
        "outline",
        "error",
    ];
}

pub struct MatugenService;

impl MatugenService {
    /// Whether `matugen` is available on the system PATH.
    pub fn is_installed() -> bool {
        Command::new("which")
            .arg("matugen")
            .output()
            .is_ok_and(|o| o.status.success())
    }

    /// Detect the current KDE Plasma wallpaper path.
    pub fn detect_wallpaper() -> Option<String> {
        let home = std::env::var("HOME").ok()?;

        // Primary source: screen locker config
        let locker_path = format!("{}/.config/kscreenlockerrc", home);
        if let Some(path) = extract_image_from_file(&locker_path) {
            return Some(path);
        }

        // Fallback: desktop appletsrc
        let desktop_path = format!(
            "{}/.config/plasma-org.kde.plasma.desktop-appletsrc",
            home
        );
        extract_image_from_file(&desktop_path)
    }

    /// Run `matugen` and parse the resulting JSON into a `MatugenPalette`.
    ///
    /// Returns `None` on any error.
    pub fn extract_palette(image_path: &str) -> Option<MatugenPalette> {
        let output = Command::new("matugen")
            .args(["image", image_path, "--json", "hex", "--prefer=saturation"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_matugen_json(&stdout, image_path)
    }

    /// Detect whether the system KDE theme is dark.
    ///
    /// Reads `~/.config/kdeglobals` → `[Colors:Window]` → `BackgroundNormal=R,G,B`
    /// and returns `true` if the luminance is below 128 (dark theme).
    /// Defaults to `true` (dark) if detection fails.
    pub fn is_system_dark() -> bool {
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return true,
        };
        let path = format!("{}/.config/kdeglobals", home);
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return true,
        };
        is_dark_from_kdeglobals(&content).unwrap_or(true)
    }
}

fn extract_image_from_file(file_path: &str) -> Option<String> {
    let content = std::fs::read_to_string(file_path).ok()?;
    extract_image_from_content(&content)
}

/// Extract the first valid `Image=` path from KDE config file content.
fn extract_image_from_content(content: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?m)^Image=(.+)$").ok()?;
    for cap in re.captures_iter(content) {
        let raw = cap.get(1)?.as_str().trim();
        let path = if let Some(stripped) = raw.strip_prefix("file://") {
            stripped.to_string()
        } else {
            raw.to_string()
        };
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

/// Parse matugen JSON output into a `MatugenPalette`.
pub fn parse_matugen_json(json: &str, image_path: &str) -> Option<MatugenPalette> {
    let data: serde_json::Value = serde_json::from_str(json).ok()?;
    let colors = data.get("colors")?.as_object()?;

    let mut dark = HashMap::new();
    let mut light = HashMap::new();

    for (name, variants) in colors {
        if let Some(variants) = variants.as_object() {
            if let Some(dark_val) = variants.get("dark").and_then(|v| v.as_object()) {
                if let Some(color) = dark_val.get("color").and_then(|c| c.as_str()) {
                    dark.insert(name.clone(), color.to_string());
                }
            }
            if let Some(light_val) = variants.get("light").and_then(|v| v.as_object()) {
                if let Some(color) = light_val.get("color").and_then(|c| c.as_str()) {
                    light.insert(name.clone(), color.to_string());
                }
            }
        }
    }

    Some(MatugenPalette {
        image_path: image_path.to_string(),
        dark,
        light,
    })
}

/// Parse a `BackgroundNormal=R,G,B` line and return whether it's dark.
pub fn is_dark_from_kdeglobals(content: &str) -> Option<bool> {
    let mut in_colors_window = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_colors_window = trimmed == "[Colors:Window]";
            continue;
        }
        if in_colors_window {
            if let Some(val) = trimmed.strip_prefix("BackgroundNormal=") {
                let parts: Vec<&str> = val.split(',').collect();
                if parts.len() >= 3 {
                    let r: f32 = parts[0].trim().parse().unwrap_or(0.0);
                    let g: f32 = parts[1].trim().parse().unwrap_or(0.0);
                    let b: f32 = parts[2].trim().parse().unwrap_or(0.0);
                    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
                    return Some(luminance < 128.0);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_colors_count() {
        assert_eq!(MatugenPalette::KEY_COLORS.len(), 18);
    }

    #[test]
    fn key_colors_include_primary() {
        assert!(MatugenPalette::KEY_COLORS.contains(&"primary"));
        assert!(MatugenPalette::KEY_COLORS.contains(&"source_color"));
        assert!(MatugenPalette::KEY_COLORS.contains(&"error"));
    }

    #[test]
    fn parse_matugen_json_basic() {
        let json = r##"{
            "colors": {
                "primary": {
                    "dark": { "color": "#d4bca0" },
                    "light": { "color": "#5a4632" }
                },
                "secondary": {
                    "dark": { "color": "#d5c3ad" },
                    "light": { "color": "#5b4a37" }
                }
            }
        }"##;
        let palette = parse_matugen_json(json, "/tmp/test.png").unwrap();
        assert_eq!(palette.image_path, "/tmp/test.png");
        assert_eq!(palette.dark.get("primary"), Some(&"#d4bca0".to_string()));
        assert_eq!(palette.light.get("primary"), Some(&"#5a4632".to_string()));
        assert_eq!(palette.dark.get("secondary"), Some(&"#d5c3ad".to_string()));
        assert_eq!(palette.dark.len(), 2);
        assert_eq!(palette.light.len(), 2);
    }

    #[test]
    fn parse_matugen_json_empty_colors() {
        let json = r#"{ "colors": {} }"#;
        let palette = parse_matugen_json(json, "/tmp/test.png").unwrap();
        assert!(palette.dark.is_empty());
        assert!(palette.light.is_empty());
    }

    #[test]
    fn parse_matugen_json_invalid() {
        assert!(parse_matugen_json("not json", "").is_none());
        assert!(parse_matugen_json("{}", "").is_none());
        assert!(parse_matugen_json(r#"{ "other": 1 }"#, "").is_none());
    }

    #[test]
    fn parse_matugen_json_partial_variants() {
        let json = r##"{
            "colors": {
                "primary": {
                    "dark": { "color": "#aabbcc" }
                }
            }
        }"##;
        let palette = parse_matugen_json(json, "img.png").unwrap();
        assert_eq!(palette.dark.get("primary"), Some(&"#aabbcc".to_string()));
        assert!(palette.light.get("primary").is_none());
    }

    #[test]
    fn is_dark_from_kdeglobals_dark_theme() {
        let content = "[Colors:Window]\nBackgroundNormal=35,38,41\n";
        assert_eq!(is_dark_from_kdeglobals(content), Some(true));
    }

    #[test]
    fn is_dark_from_kdeglobals_light_theme() {
        let content = "[Colors:Window]\nBackgroundNormal=239,240,241\n";
        assert_eq!(is_dark_from_kdeglobals(content), Some(false));
    }

    #[test]
    fn is_dark_from_kdeglobals_missing_section() {
        let content = "[General]\nfoo=bar\n";
        assert_eq!(is_dark_from_kdeglobals(content), None);
    }

    #[test]
    fn is_dark_from_kdeglobals_wrong_section() {
        let content = "[Colors:Button]\nBackgroundNormal=35,38,41\n";
        assert_eq!(is_dark_from_kdeglobals(content), None);
    }

    #[test]
    fn is_dark_from_kdeglobals_multiple_sections() {
        let content = "[General]\nfoo=bar\n[Colors:Window]\nBackgroundNormal=200,200,200\n[Other]\n";
        assert_eq!(is_dark_from_kdeglobals(content), Some(false));
    }

    #[test]
    fn extract_image_from_content_file_url() {
        // Uses a path that actually exists
        let content = format!("Image=file:///proc/self/exe\n");
        let result = extract_image_from_content(&content);
        assert_eq!(result, Some("/proc/self/exe".to_string()));
    }

    #[test]
    fn extract_image_from_content_plain_path() {
        let content = "Image=/proc/self/exe\n";
        let result = extract_image_from_content(&content);
        assert_eq!(result, Some("/proc/self/exe".to_string()));
    }

    #[test]
    fn extract_image_from_content_nonexistent() {
        let content = "Image=/nonexistent/path/wallpaper.png\n";
        assert!(extract_image_from_content(content).is_none());
    }

    #[test]
    fn extract_image_from_content_no_image_key() {
        let content = "[Desktop]\nWallpaper=test.png\n";
        assert!(extract_image_from_content(content).is_none());
    }
}
