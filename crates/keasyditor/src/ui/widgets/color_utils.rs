//! Shared color utility functions used across the UI.

use std::collections::HashMap;

use iced::Color;

/// Parse a hex color string like "#ff0000" or "ff0000" into an iced `Color`.
/// Returns a fallback gray if parsing fails.
pub fn parse_hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return Color::from_rgb(0.5, 0.5, 0.5);
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

/// Read a color from text_input_values hex key, falling back to slider R/G/B, then default.
///
/// Key format: hex key = `"color.<name>.hex"`, slider keys = `"color.<name>.r/g/b"`.
pub fn read_color(
    name: &str,
    text_input_values: &HashMap<String, String>,
    slider_values: &HashMap<String, f32>,
    default: Color,
) -> Color {
    let hex_key = format!("color.{}.hex", name);
    if let Some(hex) = text_input_values.get(&hex_key)
        && hex.len() >= 4 {
            return parse_hex(hex);
        }
    let r_key = format!("color.{}.r", name);
    let g_key = format!("color.{}.g", name);
    let b_key = format!("color.{}.b", name);
    if let (Some(&r), Some(&g), Some(&b)) = (
        slider_values.get(&r_key),
        slider_values.get(&g_key),
        slider_values.get(&b_key),
    ) {
        return Color::from_rgb(
            (r / 255.0).clamp(0.0, 1.0),
            (g / 255.0).clamp(0.0, 1.0),
            (b / 255.0).clamp(0.0, 1.0),
        );
    }
    default
}

/// Calculate perceived luminance (0.0 = black, 1.0 = white).
pub fn luminance(c: Color) -> f32 {
    0.299 * c.r + 0.587 * c.g + 0.114 * c.b
}

/// Blend two colors: `a * (1-t) + b * t`.
pub fn blend(a: Color, b: Color, t: f32) -> Color {
    Color::from_rgb(
        (a.r + (b.r - a.r) * t).clamp(0.0, 1.0),
        (a.g + (b.g - a.g) * t).clamp(0.0, 1.0),
        (a.b + (b.b - a.b) * t).clamp(0.0, 1.0),
    )
}

/// Apply contrast, intensity, and saturation adjustments to a color.
/// Each factor: 1.0 = neutral, <1.0 = reduce, >1.0 = increase.
pub fn adjust_color(c: Color, contrast: f32, intensity: f32, saturation: f32) -> Color {
    let r = (c.r * intensity).clamp(0.0, 1.0);
    let g = (c.g * intensity).clamp(0.0, 1.0);
    let b = (c.b * intensity).clamp(0.0, 1.0);

    let gray = 0.299 * r + 0.587 * g + 0.114 * b;
    let r = (gray + (r - gray) * saturation).clamp(0.0, 1.0);
    let g = (gray + (g - gray) * saturation).clamp(0.0, 1.0);
    let b = (gray + (b - gray) * saturation).clamp(0.0, 1.0);

    let r = (0.5 + (r - 0.5) * contrast).clamp(0.0, 1.0);
    let g = (0.5 + (g - 0.5) * contrast).clamp(0.0, 1.0);
    let b = (0.5 + (b - 0.5) * contrast).clamp(0.0, 1.0);

    Color::from_rgb(r, g, b)
}
