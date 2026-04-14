/// Service for applying a matugen-extracted wallpaper palette system-wide.
///
/// Overwrites the currently-active Kvantum theme's `[GeneralColors]` with
/// the matugen palette (dark or light variant per the user's selection),
/// rewrites Klassy color fields, and refreshes the Plasma color scheme
/// derived from the rewritten Kvantum theme. System themes are cloned to
/// the user directory on first use so we never need write access to
/// `/usr/share/Kvantum/`.
use std::collections::HashMap;
use std::io;

use crate::color::{self, Rgba};
use crate::constants;
use crate::models::kvantum::colors::KvantumColors;
use crate::services::{
    FileService, KlassyService, KvantumService, MatugenPalette, ProcessService,
};

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

    /// Rewrite the currently-active Kvantum theme's `[GeneralColors]` with
    /// the matugen palette variant selected by `prefer_dark`, update
    /// Klassy colors, and refresh the Plasma color scheme derived from
    /// that theme.
    ///
    /// The user's **active system Kvantum theme** (`kvantum.kvconfig →
    /// theme=<name>`) is the target. If it's a system theme (lives under
    /// `/usr/share/Kvantum/`), it's cloned to the user directory first
    /// so we don't need write access to `/usr/share`. The theme's own
    /// SVG is recolored in place via [`recolor_svg_for_palette`] so the
    /// visual design stays but all greys tint toward the new window/text
    /// colors and accents adopt the new highlight hue.
    ///
    /// **This is destructive**: the active theme's original palette is
    /// lost. Callers should confirm with the user before invoking.
    pub fn apply(
        &self,
        palette: &MatugenPalette,
        prefer_dark: bool,
    ) -> WallpaperApplyOutcome {
        let mut steps = Vec::new();
        let mut ok = true;

        let variant = if prefer_dark { &palette.dark } else { &palette.light };

        // 1. Rewrite Klassy colors.
        match self.apply_to_klassy(variant) {
            Ok(()) => steps.push("Updated Klassy colors".to_string()),
            Err(e) => {
                ok = false;
                steps.push(format!("Failed to update Klassy: {}", e));
            }
        }

        // 2. Identify the active Kvantum theme.
        let theme_name = match self.kvantum.get_active_theme() {
            Some(n) => n,
            None => {
                steps.push(
                    "No active Kvantum theme to overwrite — activate a theme first".to_string(),
                );
                return WallpaperApplyOutcome { steps, ok: false };
            }
        };

        // 3. Load the theme (user dir first, then system dir fallback).
        //    System themes get promoted to the user directory on save.
        let user_dir = format!(
            "{}/{}",
            constants::kvantum_config_dir().to_string_lossy(),
            theme_name
        );
        let system_dir = format!(
            "{}/{}",
            constants::kvantum_system_dir().to_string_lossy(),
            theme_name
        );
        let mut theme = match self.kvantum.load_theme(&user_dir) {
            Ok(t) => t,
            Err(_) => match self.kvantum.load_theme(&system_dir) {
                Ok(t) => t,
                Err(e) => {
                    steps.push(format!(
                        "Failed to load active theme '{}': {}",
                        theme_name, e
                    ));
                    return WallpaperApplyOutcome { steps, ok: false };
                }
            },
        };

        // 4. Overwrite `[GeneralColors]` from the matugen variant.
        for (key, matugen_key) in kvantum_color_mapping() {
            if let Some(rgba) = lookup_color(variant, matugen_key) {
                theme.config.colors.set_color(key, rgba);
            }
        }

        // 5. Ensure the theme has a renderable SVG + widget section
        //    definitions, then recolor the SVG. Chromatic accents adopt
        //    the palette's highlight hue and grayscale pixels
        //    interpolate between window and text.
        //
        //    Kvantum falls back to its built-in *dark* default render
        //    for any theme whose `.kvconfig` lacks the per-widget
        //    sections (`[PanelButtonCommand]`, `[Tab]`, etc.) that
        //    reference SVG elements — regardless of whether an SVG
        //    file is present. A bare theme like `[%General]` +
        //    `[GeneralColors]` + `[Hacks]` with no widget sections
        //    renders as solid dark gray even with a valid SVG on disk.
        //    So whenever the theme has no widget sections OR no SVG,
        //    we inject KvAdapta / KvAdaptaDark's full config structure
        //    and SVG, then overlay our matugen palette on top.
        let needs_template =
            theme.svg_content.is_none() || theme.config.widget_sections.is_empty();
        let template = if needs_template {
            load_adaptive_template(prefer_dark, &self.kvantum)
        } else {
            None
        };

        let source_svg = theme
            .svg_content
            .take()
            .or_else(|| template.as_ref().and_then(|t| t.svg_content.clone()));
        if let Some(svg) = source_svg {
            theme.svg_content = Some(recolor_svg_for_palette(&svg, &theme.config.colors));
        }

        // If the theme lacked widget sections, adopt the template's
        // full `[%General]` + widget sections while keeping our
        // matugen-overwritten colors. `[Hacks]` comes from the
        // template if the theme had none of its own.
        if let Some(tpl) = template
            && theme.config.widget_sections.is_empty()
        {
            let our_colors = theme.config.colors.clone();
            let our_hacks = if theme.config.hacks.is_empty() {
                tpl.config.hacks.clone()
            } else {
                theme.config.hacks.clone()
            };
            theme.config = tpl
                .config
                .copy_with_colors(our_colors)
                .copy_with_hacks(our_hacks);
        }

        // 6. Save to the user directory. `save_theme` derives file names
        //    from the target dir's basename, so we must point at user_dir
        //    (not system_dir) and set `theme_name` + `dir_path` so they
        //    stay consistent if the caller inspects the return.
        theme.theme_name = theme_name.clone();
        theme.dir_path = user_dir.clone();
        match self.kvantum.save_theme(&user_dir, &theme) {
            Ok(()) => steps.push(format!(
                "Overwrote '[GeneralColors]' of '{}' at {}",
                theme_name, user_dir
            )),
            Err(e) => {
                ok = false;
                steps.push(format!("Failed to save '{}': {}", theme_name, e));
            }
        }

        // 7. Re-apply the Kvantum theme so its Qt style plugin reloads
        //    the rewritten palette. `kvantummanager --set` on the
        //    already-active name still writes `kvantum.kvconfig` and
        //    triggers a reload for new Qt processes.
        match self.process.apply_kvantum_theme(&theme_name) {
            Ok(r) if r.is_success() => {
                steps.push(format!("Re-activated Kvantum theme '{}'", theme_name));
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

        // 8. Sync the Plasma color scheme derived from the freshly-
        //    rewritten Kvantum theme, so KDE's Colors panel and KWin
        //    decorations pick up the new palette.
        match sync_plasma_scheme_from_kvantum_name(&theme_name) {
            Ok(outcome) => {
                steps.push(format!(
                    "Derived Plasma color scheme '{}' from Kvantum theme",
                    outcome.scheme_name
                ));
            }
            Err(e) => {
                ok = false;
                steps.push(format!("Plasma color scheme sync failed: {}", e));
            }
        }

        // 9. Reconfigure KWin so Klassy decoration changes take effect.
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
}

/// Load a system-installed adaptive theme (KvAdapta/KvAdaptaDark and a
/// couple of fallbacks) as a full template — SVG + widget section
/// definitions — for themes that are structurally incomplete. Returns
/// `None` if no candidate is installed or all candidates are missing
/// an SVG.
///
/// Kvantum refuses to render a theme's SVG unless the `.kvconfig` also
/// declares the per-widget sections (`[PanelButtonCommand]`, `[Tab]`,
/// ...) that reference SVG element IDs. So we need the template's full
/// config, not just its SVG bytes.
///
/// `KvAdapta` is the most "palette-neutral" adaptive theme shipped
/// with Kvantum — its SVG paths use mostly grayscale fills that our
/// [`recolor_svg_for_palette`] can retint cleanly.
fn load_adaptive_template(
    dark: bool,
    kvantum: &KvantumService,
) -> Option<crate::services::KvantumThemeData> {
    const CANDIDATES_DARK: &[&str] = &[
        "/usr/share/Kvantum/KvAdaptaDark",
        "/usr/share/Kvantum/KvDark",
        "/usr/share/Kvantum/KvAdapta",
    ];
    const CANDIDATES_LIGHT: &[&str] = &[
        "/usr/share/Kvantum/KvAdapta",
        "/usr/share/Kvantum/KvFlat",
        "/usr/share/Kvantum/KvBeige",
    ];
    let candidates = if dark { CANDIDATES_DARK } else { CANDIDATES_LIGHT };
    for dir in candidates {
        if let Ok(theme) = kvantum.load_theme(dir)
            && theme.svg_content.is_some()
            && !theme.config.widget_sections.is_empty()
        {
            return Some(theme);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// SVG recoloring: substitute source colors with matugen palette colors
// ---------------------------------------------------------------------------

/// Rewrite every `#RRGGBB` and `#RGB` color literal in `svg` according
/// to the `[GeneralColors]` palette:
///
/// - **Chromatic** source colors (saturation > 0.25) are replaced with
///   `highlight.color`, except purple-ish hues (hue 260°–320°) which go
///   to `link.visited.color` so the secondary accent stays distinct.
/// - **Grayscale** source colors (saturation ≤ 0.25) are interpolated
///   between `window.color` (luminance ≈ 1) and `window.text.color`
///   (luminance ≈ 0) according to the source's luminance. This preserves
///   the template's light/dark hierarchy while retinting it to our palette.
///
/// Both 6-char (`#ffcc00`) and 3-char (`#fc0`) forms are handled. The
/// 3-char form is especially important because KvAdapta's template uses
/// `#fff` and `#000` heavily for widget backgrounds — missing them leaves
/// the dark-theme variant looking "inverted" (white content on a dark
/// shell).
pub fn recolor_svg_for_palette(svg: &str, colors: &KvantumColors) -> String {
    use std::collections::HashSet;

    let window = colors.window_color();
    let text = colors.window_text_color();
    let highlight = colors.highlight_color();
    let visited = colors.link_visited_color();

    // Collect unique 6-char hex literals first, then unique 3-char ones.
    // The 3-char regex has a negative look-around emulated by checking
    // that the match isn't immediately followed by another hex digit —
    // so we don't accidentally truncate `#abcdef` into `#abc` + `def`.
    let re6 = regex::Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
    let re3 = regex::Regex::new(r"#[0-9a-fA-F]{3}").unwrap();

    let six: HashSet<&str> = re6.find_iter(svg).map(|m| m.as_str()).collect();

    let mut out = svg.to_string();

    // Pass 1: replace 6-char literals.
    for src_str in &six {
        let Some(src) = parse_six_hex(src_str) else { continue };
        let dst = recolor_pixel(src, window, text, highlight, visited);
        let dst_str = format!("#{:02x}{:02x}{:02x}", dst.r, dst.g, dst.b);
        if dst_str.eq_ignore_ascii_case(src_str) {
            continue;
        }
        out = out.replace(src_str, &dst_str);
    }

    // Pass 2: replace 3-char literals. We need to iterate against the
    // ALREADY-UPDATED string so the 6-char pass's output doesn't get
    // mis-matched as 3-char. We collect matches with their byte ranges,
    // filter out any that are followed by another hex digit (which would
    // mean they're a prefix of a 6-char literal — shouldn't happen after
    // pass 1, but defensive), then substitute back-to-front to keep
    // offsets stable.
    let bytes = out.as_bytes();
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    for m in re3.find_iter(&out) {
        let end = m.end();
        // If next byte is another hex digit, this is a prefix of a
        // longer literal — skip.
        if end < bytes.len() && (bytes[end] as char).is_ascii_hexdigit() {
            continue;
        }
        let src_str = m.as_str();
        let Some(src) = parse_three_hex(src_str) else { continue };
        let dst = recolor_pixel(src, window, text, highlight, visited);
        let dst_str = format!("#{:02x}{:02x}{:02x}", dst.r, dst.g, dst.b);
        replacements.push((m.start(), end, dst_str));
    }
    // Apply in reverse so earlier offsets stay correct.
    for (start, end, dst_str) in replacements.into_iter().rev() {
        out.replace_range(start..end, &dst_str);
    }

    out
}

/// Parse a `#RGB` shorthand literal into `Rgba` by doubling each nibble.
/// `#abc` → `(0xaa, 0xbb, 0xcc)`. Returns `None` for malformed input.
fn parse_three_hex(s: &str) -> Option<Rgba> {
    if s.len() != 4 || !s.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&s[1..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..3], 16).ok()?;
    let b = u8::from_str_radix(&s[3..4], 16).ok()?;
    Some(Rgba::rgb(r * 17, g * 17, b * 17))
}

/// Map a single source RGB pixel through the palette substitution rules.
/// Public for testability.
///
/// For grayscale pixels, the interpolation is oriented by the POLARITY
/// of the target palette (window vs text). The invariant is:
///   - `source_bg` (the luminance-dominant end of the source template)
///     maps to `window.color`
///   - `source_fg` (the other end) maps to `window.text.color`
///
/// This means in a LIGHT theme (window bright, text dark), a white
/// source pixel maps to window (bright) and a black source pixel maps
/// to text (dark). In a DARK theme (window dark, text bright) the
/// assignment flips: white source → text (bright), black source →
/// window (dark). The template's light/dark hierarchy is preserved in
/// the palette's own polarity rather than blindly inverted.
pub fn recolor_pixel(
    src: Rgba,
    window: Rgba,
    text: Rgba,
    highlight: Rgba,
    visited: Rgba,
) -> Rgba {
    let (h, s, l) = rgb_to_hsl(src);

    if s > 0.25 {
        // Chromatic accent — branch on hue. Purple/magenta (≈260°–320°)
        // maps to the secondary accent; everything else to the primary.
        if (260.0..=320.0).contains(&h) {
            return visited;
        }
        return highlight;
    }

    // Grayscale. Decide polarity: if the palette's `window` is brighter
    // than its `text` (a LIGHT theme), keep the natural mapping
    // "white→window, black→text". If it's darker (a DARK theme), flip
    // so "white→text, black→window" — this keeps bright template areas
    // bright and dark template areas dark within the palette's polarity.
    let (_, _, window_l) = rgb_to_hsl(window);
    let (_, _, text_l) = rgb_to_hsl(text);
    let light_palette = window_l >= text_l;

    // `t` is how much of the "text end" to mix in. For a light palette,
    // t=0 at l=1 (white source → window); for a dark palette, t=1 at
    // l=1 (white source → text).
    let t = if light_palette { 1.0 - l } else { l };

    let mix = |a: u8, b: u8| -> u8 {
        (a as f32 * (1.0 - t) + b as f32 * t).clamp(0.0, 255.0) as u8
    };
    Rgba::rgb(
        mix(window.r, text.r),
        mix(window.g, text.g),
        mix(window.b, text.b),
    )
}

/// Parse an exact `#RRGGBB` literal (no `#RGB`, no alpha) into `Rgba`.
fn parse_six_hex(s: &str) -> Option<Rgba> {
    if s.len() != 7 || !s.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&s[1..3], 16).ok()?;
    let g = u8::from_str_radix(&s[3..5], 16).ok()?;
    let b = u8::from_str_radix(&s[5..7], 16).ok()?;
    Some(Rgba::rgb(r, g, b))
}

/// Convert 8-bit sRGB to HSL. Returns `(hue_deg, saturation, lightness)`
/// where hue ∈ [0, 360), saturation ∈ [0, 1], lightness ∈ [0, 1].
fn rgb_to_hsl(c: Rgba) -> (f32, f32, f32) {
    let r = c.r as f32 / 255.0;
    let g = c.g as f32 / 255.0;
    let b = c.b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / d + 2.0) * 60.0
    } else {
        ((r - g) / d + 4.0) * 60.0
    };
    (h, s, l)
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

/// Resolve a matugen color name to an `Rgba`. Falls back to `None` if missing
/// or unparseable.
fn lookup_color(variant: &HashMap<String, String>, key: &str) -> Option<Rgba> {
    variant.get(key).and_then(|v| color::try_parse(v))
}

// ---------------------------------------------------------------------------
// Kvantum → Plasma color scheme synchronization
// ---------------------------------------------------------------------------

/// The Plasma color scheme name we install for a given Kvantum theme.
pub fn plasma_scheme_name_for_kvantum(theme_name: &str) -> String {
    let sanitized: String = theme_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    format!("Kvantum-{}", sanitized)
}

/// Build a Plasma `.colors` file content from a Kvantum theme's
/// `[GeneralColors]` palette.
///
/// IMPORTANT: Kvantum's `[GeneralColors]` keys map to Qt's `QPalette` roles,
/// which are NOT literal "light/dark shade" names:
///   - `light.color` / `mid.light.color` / `mid.color` / `dark.color` are
///     3D shading roles (bezel highlights/shadows), relative to Button.
///     They are frequently set to values that look wrong as a flat surface
///     background — e.g. Teto uses `light.color=#22191b` (near-black).
///     DO NOT use them as KDE section backgrounds.
///   - `window.color` / `base.color` / `alt.base.color` / `button.color`
///     are the actual surface colors.
///
/// The mapping below only uses the surface colors for backgrounds.
pub fn build_plasma_scheme_from_kvantum(scheme_name: &str, colors: &KvantumColors) -> String {
    let rgba = |key: &str| -> Rgba { colors.get_color(key).unwrap_or(Rgba::rgb(0, 0, 0)) };
    let rgb = |c: Rgba| format!("{},{},{}", c.r, c.g, c.b);

    let window = rgba("window.color");
    let window_fg = rgba("window.text.color");
    let base = rgba("base.color");
    let alt_base = rgba("alt.base.color");
    let text_fg = rgba("text.color");
    let button = rgba("button.color");
    let button_fg = rgba("button.text.color");
    let highlight = rgba("highlight.color");
    let highlight_fg = rgba("highlight.text.color");
    let tooltip_fg = rgba("tooltip.text.color");
    let disabled_fg = rgba("disabled.text.color");
    let link = rgba("link.color");
    let visited = rgba("link.visited.color");
    let inactive_selection = rgba("inactive.highlight.color");

    // Derive secondary surfaces from `window`/`base` so we don't accidentally
    // pick up the 3D shading roles. `toward_fg` nudges the background a few
    // percent toward the text color — this gives us a distinct "header" or
    // "tooltip" surface without making it inverted.
    let toward_fg = |bg: Rgba, fg: Rgba, t: f32| -> Rgba {
        let mix = |a: u8, b: u8| -> u8 {
            let v = a as f32 + (b as f32 - a as f32) * t;
            v.clamp(0.0, 255.0) as u8
        };
        Rgba::rgb(mix(bg.r, fg.r), mix(bg.g, fg.g), mix(bg.b, fg.b))
    };

    let header_bg = toward_fg(window, window_fg, 0.08);
    let tooltip_bg = toward_fg(window, window_fg, 0.15);
    let complementary_bg = toward_fg(window, window_fg, 0.04);
    let window_alt = toward_fg(window, window_fg, 0.04);
    let button_alt = toward_fg(button, button_fg, 0.04);

    let window_s = rgb(window);
    let window_fg_s = rgb(window_fg);
    let base_s = rgb(base);
    let alt_base_s = rgb(alt_base);
    let text_fg_s = rgb(text_fg);
    let button_s = rgb(button);
    let button_fg_s = rgb(button_fg);
    let highlight_s = rgb(highlight);
    let highlight_fg_s = rgb(highlight_fg);
    let tooltip_fg_s = rgb(tooltip_fg);
    let disabled_fg_s = rgb(disabled_fg);
    let link_s = rgb(link);
    let visited_s = rgb(visited);
    let inactive_selection_s = rgb(inactive_selection);
    let header_bg_s = rgb(header_bg);
    let tooltip_bg_s = rgb(tooltip_bg);
    let complementary_bg_s = rgb(complementary_bg);
    let window_alt_s = rgb(window_alt);
    let button_alt_s = rgb(button_alt);

    // Helper: write one `[Colors:*]` section with full 12-key payload.
    // `fg_active` is the "active state" foreground (hover/focused text) —
    // using `highlight.color` (a chromatic accent) so it reads as foreground,
    // not the selection BACKGROUND we were using before.
    let section = |name: &str, bg: &str, alt_bg: &str, fg: &str| -> String {
        format!(
            "[{name}]\n\
             BackgroundNormal={bg}\n\
             BackgroundAlternate={alt_bg}\n\
             ForegroundNormal={fg}\n\
             ForegroundActive={highlight_s}\n\
             ForegroundInactive={disabled_fg_s}\n\
             ForegroundLink={link_s}\n\
             ForegroundVisited={visited_s}\n\
             ForegroundNegative=237,21,21\n\
             ForegroundNeutral=246,116,0\n\
             ForegroundPositive=39,174,96\n\
             DecorationFocus={highlight_s}\n\
             DecorationHover={highlight_s}\n\n",
        )
    };

    let mut out = String::new();
    out.push_str("[ColorEffects:Disabled]\n");
    out.push_str(
        "Color=56,56,56\nColorAmount=0\nColorEffect=0\nContrastAmount=0.65\nContrastEffect=1\nIntensityAmount=0.1\nIntensityEffect=2\n\n",
    );
    out.push_str("[ColorEffects:Inactive]\n");
    out.push_str(
        "ChangeSelectionColor=true\nColor=112,111,110\nColorAmount=0.025\nColorEffect=2\nContrastAmount=0.1\nContrastEffect=2\nEnable=false\nIntensityAmount=0\nIntensityEffect=0\n\n",
    );

    out.push_str(&section("Colors:Window", &window_s, &window_alt_s, &window_fg_s));
    out.push_str(&section("Colors:Button", &button_s, &button_alt_s, &button_fg_s));
    out.push_str(&section("Colors:View", &base_s, &alt_base_s, &text_fg_s));
    out.push_str(&section(
        "Colors:Selection",
        &highlight_s,
        &inactive_selection_s,
        &highlight_fg_s,
    ));
    out.push_str(&section(
        "Colors:Tooltip",
        &tooltip_bg_s,
        &window_alt_s,
        &tooltip_fg_s,
    ));
    out.push_str(&section(
        "Colors:Complementary",
        &complementary_bg_s,
        &window_alt_s,
        &window_fg_s,
    ));
    out.push_str(&section(
        "Colors:Header",
        &header_bg_s,
        &window_alt_s,
        &window_fg_s,
    ));

    out.push_str(&format!("[General]\nName={}\nshadeSortColumn=true\n\n", scheme_name));
    out.push_str("[KDE]\ncontrast=4\n\n");
    // Titlebar: active uses the header surface (slight tint of window),
    // inactive uses a lighter blend toward disabled_fg so it reads as dimmer.
    let wm_inactive_bg = toward_fg(window, disabled_fg, 0.25);
    out.push_str(&format!(
        "[WM]\nactiveBackground={}\nactiveForeground={}\ninactiveBackground={}\ninactiveForeground={}\n",
        header_bg_s,
        window_fg_s,
        rgb(wm_inactive_bg),
        disabled_fg_s,
    ));

    out
}

/// Outcome of `sync_plasma_scheme_from_kvantum`.
#[derive(Debug, Clone)]
pub struct KvantumPlasmaSyncOutcome {
    /// Absolute path of the installed `.colors` file.
    pub scheme_path: String,
    /// Name of the Plasma color scheme (e.g. `Kvantum-Teto`).
    pub scheme_name: String,
}

/// Load the Kvantum theme at `theme_dir_path`, derive a matching Plasma color
/// scheme from its `[GeneralColors]` section, install it to
/// `~/.local/share/color-schemes/`, update `ColorSchemeHash` in `kdeglobals`
/// to the new file's SHA-1, then force-apply via `plasma-apply-colorscheme`
/// (routing through an intermediate scheme so the tool actually broadcasts).
///
/// The hash update + forced apply are both needed because `plasma-apply-
/// colorscheme` treats a re-apply of the already-active scheme as a no-op
/// and never updates `ColorSchemeHash` itself — so running Qt apps would
/// never notice that the underlying `.colors` file was rewritten.
pub fn sync_plasma_scheme_from_kvantum_dir(
    theme_dir_path: &str,
) -> Result<KvantumPlasmaSyncOutcome, String> {
    let kv_svc = KvantumService::new(FileService::new());
    let theme = kv_svc.load_theme(theme_dir_path).map_err(|e| e.to_string())?;

    let scheme_name = plasma_scheme_name_for_kvantum(&theme.theme_name);
    let content = build_plasma_scheme_from_kvantum(&scheme_name, &theme.config.colors);

    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let dir = format!("{}/.local/share/color-schemes", home);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = format!("{}/{}.colors", dir, scheme_name);
    std::fs::write(&path, &content).map_err(|e| e.to_string())?;

    let proc_svc = ProcessService::new();

    // 1. Compute SHA-1 of the just-written file and write it into
    //    `kdeglobals` so running Qt apps detect a fresh hash on next refresh.
    let hash = sha1_hex(content.as_bytes());
    let _ = proc_svc.write_kdeglobals_colorscheme_hash(&hash);

    // 2. Force-apply through an intermediate scheme so `plasma-apply-
    //    colorscheme` actually broadcasts `KGlobalSettings.notifyChange`
    //    even when `scheme_name` is already the active scheme.
    let result = proc_svc
        .apply_plasma_colorscheme_forced(&scheme_name)
        .map_err(|e| e.to_string())?;
    if !result.is_success() {
        return Err(format!(
            "plasma-apply-colorscheme failed: {}",
            result.stderr.trim()
        ));
    }

    Ok(KvantumPlasmaSyncOutcome {
        scheme_path: path,
        scheme_name,
    })
}

/// Lowercase hex SHA-1 of the given bytes.
fn sha1_hex(data: &[u8]) -> String {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(40);
    for b in digest.iter() {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

/// Convenience wrapper: look up the Kvantum theme directory for `theme_name`
/// in either the user or system Kvantum config root, then sync the Plasma
/// color scheme. Tries user path first so user overrides win.
pub fn sync_plasma_scheme_from_kvantum_name(
    theme_name: &str,
) -> Result<KvantumPlasmaSyncOutcome, String> {
    let user_path = format!(
        "{}/{}",
        constants::kvantum_config_dir().to_string_lossy(),
        theme_name
    );
    if std::path::Path::new(&user_path).is_dir() {
        return sync_plasma_scheme_from_kvantum_dir(&user_path);
    }

    let system_path = format!(
        "{}/{}",
        constants::kvantum_system_dir().to_string_lossy(),
        theme_name
    );
    if std::path::Path::new(&system_path).is_dir() {
        return sync_plasma_scheme_from_kvantum_dir(&system_path);
    }

    Err(format!(
        "Kvantum theme '{}' not found in user or system directory",
        theme_name
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn rgb_to_hsl_pure_colors() {
        // Pure red: H=0, S=1, L=0.5
        let (h, s, l) = rgb_to_hsl(Rgba::rgb(255, 0, 0));
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);

        // Pure white: H=any, S=0, L=1
        let (_, s, l) = rgb_to_hsl(Rgba::rgb(255, 255, 255));
        assert!(s < 0.01);
        assert!((l - 1.0).abs() < 0.01);

        // Pure black: H=any, S=0, L=0
        let (_, s, l) = rgb_to_hsl(Rgba::rgb(0, 0, 0));
        assert!(s < 0.01);
        assert!(l < 0.01);

        // KvAdapta teal #00bcd4: hue ≈ 187°, saturated
        let (h, s, _) = rgb_to_hsl(Rgba::rgb(0x00, 0xbc, 0xd4));
        assert!((180.0..=200.0).contains(&h));
        assert!(s > 0.5);
    }

    #[test]
    fn recolor_pixel_saturated_maps_to_highlight() {
        // KvAdapta teal — chromatic, non-purple → highlight
        let src = Rgba::rgb(0x00, 0xbc, 0xd4);
        let window = Rgba::rgb(255, 248, 248);
        let text = Rgba::rgb(34, 25, 27);
        let highlight = Rgba::rgb(140, 74, 95);
        let visited = Rgba::rgb(124, 86, 52);
        let out = recolor_pixel(src, window, text, highlight, visited);
        assert_eq!(out, highlight);
    }

    #[test]
    fn recolor_pixel_purple_maps_to_visited() {
        // KvAdapta purple #b74aff — chromatic, purple hue → visited
        let src = Rgba::rgb(0xb7, 0x4a, 0xff);
        let window = Rgba::rgb(255, 248, 248);
        let text = Rgba::rgb(34, 25, 27);
        let highlight = Rgba::rgb(140, 74, 95);
        let visited = Rgba::rgb(124, 86, 52);
        let out = recolor_pixel(src, window, text, highlight, visited);
        assert_eq!(out, visited);
    }

    #[test]
    fn recolor_pixel_light_palette_maps_white_to_window() {
        // LIGHT palette: window (bright) ← white source, text (dark) ← black source.
        let window = Rgba::rgb(255, 248, 248);
        let text = Rgba::rgb(34, 25, 27);
        let highlight = Rgba::rgb(140, 74, 95);
        let visited = Rgba::rgb(124, 86, 52);

        let white_out = recolor_pixel(Rgba::rgb(255, 255, 255), window, text, highlight, visited);
        assert_eq!(white_out, window, "white source → window in light palette");

        let black_out = recolor_pixel(Rgba::rgb(0, 0, 0), window, text, highlight, visited);
        assert_eq!(black_out, text, "black source → text in light palette");

        // 50% gray ≈ midpoint
        let mid_out = recolor_pixel(Rgba::rgb(128, 128, 128), window, text, highlight, visited);
        let expected_r = ((window.r as f32 + text.r as f32) / 2.0) as u8;
        assert!((mid_out.r as i16 - expected_r as i16).abs() <= 2);
    }

    #[test]
    fn recolor_pixel_dark_palette_flips_polarity() {
        // DARK palette: window (dark) ← white source, text (bright) ← black source.
        // The assignment is flipped so template brightness stays within
        // the palette's polarity — bright template regions end up as
        // text (the bright end) instead of inverting into window (dark).
        let window = Rgba::rgb(34, 25, 27);       // dark
        let text = Rgba::rgb(241, 222, 221);      // light
        let highlight = Rgba::rgb(212, 188, 160);
        let visited = Rgba::rgb(213, 190, 174);

        let white_out = recolor_pixel(Rgba::rgb(255, 255, 255), window, text, highlight, visited);
        assert_eq!(
            white_out, text,
            "white source should map to text in a dark palette (brightness preserved)"
        );

        let black_out = recolor_pixel(Rgba::rgb(0, 0, 0), window, text, highlight, visited);
        assert_eq!(
            black_out, window,
            "black source should map to window in a dark palette (darkness preserved)"
        );
    }

    #[test]
    fn recolor_svg_replaces_kvadapta_accents() {
        let svg = r##"<svg><rect fill="#00bcd4"/><rect fill="#e8e9ea"/><text fill="#1e282d">x</text></svg>"##;
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#fff8f8".to_string());
        colors.set_value("window.text.color", "#22191b".to_string());
        colors.set_value("highlight.color", "#8c4a5f".to_string());
        colors.set_value("link.visited.color", "#7c5634".to_string());

        let out = recolor_svg_for_palette(svg, &colors);
        // Teal replaced by highlight (dusty rose).
        assert!(!out.contains("#00bcd4"));
        assert!(out.contains("#8c4a5f"));
        // Near-white grayscale mapped close to window (which is #fff8f8).
        assert!(!out.contains("#e8e9ea"));
        // Near-black grayscale mapped close to text (which is #22191b).
        assert!(!out.contains("#1e282d"));
    }

    #[test]
    fn parse_three_hex_doubles_nibbles() {
        assert_eq!(parse_three_hex("#fff"), Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(parse_three_hex("#000"), Some(Rgba::rgb(0, 0, 0)));
        assert_eq!(parse_three_hex("#f0c"), Some(Rgba::rgb(0xff, 0x00, 0xcc)));
        assert_eq!(parse_three_hex("#abc"), Some(Rgba::rgb(0xaa, 0xbb, 0xcc)));
        assert_eq!(parse_three_hex("#ff0000"), None); // wrong length
        assert_eq!(parse_three_hex("fff"), None); // missing hash
    }

    #[test]
    fn recolor_svg_handles_short_hex_literals() {
        // Mixes 6-char and 3-char forms to ensure both are recolored and
        // that 3-char isn't accidentally matched as a prefix of 6-char.
        let svg = r##"<svg>
            <rect fill="#fff"/>
            <path style="fill:#000"/>
            <path style="fill:#123456"/>
            <rect fill="#fedcba"/>
        </svg>"##;
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#101010".to_string()); // dark theme
        colors.set_value("window.text.color", "#eeeeee".to_string());
        colors.set_value("highlight.color", "#ff00aa".to_string());
        colors.set_value("link.visited.color", "#00aaff".to_string());

        let out = recolor_svg_for_palette(svg, &colors);

        // 3-char literals must be gone.
        assert!(!out.contains("#fff"), "#fff should have been replaced: {}", out);
        assert!(!out.contains("#000"), "#000 should have been replaced: {}", out);
        // 6-char literals from the source must also be gone.
        assert!(!out.contains("#123456"));
        assert!(!out.contains("#fedcba"));

        // In a DARK palette the polarity flips:
        //   - `#fff` (l=1) → text (bright end)   = #eeeeee
        //   - `#000` (l=0) → window (dark end)   = #101010
        assert!(out.contains("#eeeeee"), "#fff → text in dark palette");
        assert!(out.contains("#101010"), "#000 → window in dark palette");
    }

    #[test]
    fn recolor_svg_does_not_truncate_six_char_into_three_char() {
        // Regression guard: make sure `#abcdef` isn't matched as `#abc`
        // followed by `def`. The result should use the 6-char substitution.
        let svg = r##"<svg><rect fill="#abcdef"/></svg>"##;
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#ffffff".to_string());
        colors.set_value("window.text.color", "#000000".to_string());
        colors.set_value("highlight.color", "#ff00ff".to_string());
        colors.set_value("link.visited.color", "#00ffff".to_string());

        let out = recolor_svg_for_palette(svg, &colors);
        // Original 6-char form must be replaced; result must not have
        // `def` leftover as plain text.
        assert!(!out.contains("#abcdef"));
        assert!(!out.contains(">def"));
        assert!(!out.contains("\"def"));
    }

    #[test]
    fn sha1_hex_matches_known_vectors() {
        // Standard SHA-1 test vectors — if these ever change, sha1 crate
        // broke something or we're producing a wrong hash length.
        assert_eq!(
            sha1_hex(b""),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
        assert_eq!(
            sha1_hex(b"abc"),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
        assert_eq!(sha1_hex(b"").len(), 40);
    }

    #[test]
    fn plasma_scheme_name_sanitizes_specials() {
        assert_eq!(plasma_scheme_name_for_kvantum("Teto"), "Kvantum-Teto");
        assert_eq!(
            plasma_scheme_name_for_kvantum("No Mans Sky"),
            "Kvantum-No-Mans-Sky"
        );
        assert_eq!(
            plasma_scheme_name_for_kvantum("theme.with.dots"),
            "Kvantum-theme-with-dots"
        );
    }

    #[test]
    fn build_plasma_scheme_from_kvantum_writes_window_bg() {
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#112233".to_string());
        colors.set_value("window.text.color", "#eeeeee".to_string());
        colors.set_value("button.color", "#445566".to_string());
        colors.set_value("highlight.color", "#778899".to_string());
        colors.set_value("disabled.text.color", "#808080".to_string());

        let content = build_plasma_scheme_from_kvantum("TestScheme", &colors);
        assert!(content.contains("Name=TestScheme"));
        assert!(content.contains("[Colors:Window]"));
        // Window background stays as window.color.
        assert!(content.contains("BackgroundNormal=17,34,51"));
        assert!(content.contains("[Colors:Selection]"));
        assert!(content.contains("BackgroundNormal=119,136,153"));
        assert!(content.contains("[WM]"));
        // WM active titlebar now uses a subtle blend toward the foreground,
        // not raw window.color, so it reads distinctly from the main window.
        assert!(content.contains("activeBackground="));
    }

    /// Extract `BackgroundNormal` RGB value from a specific `[Colors:*]`
    /// section within a generated `.colors` file content. Panics on missing
    /// section or malformed line — test-only, fail-fast is fine.
    fn section_bg_rgb(content: &str, section: &str) -> (u8, u8, u8) {
        let sec_idx = content
            .find(&format!("[{}]", section))
            .unwrap_or_else(|| panic!("section [{}] missing", section));
        let rest = &content[sec_idx..];
        let line = rest
            .lines()
            .find(|l| l.starts_with("BackgroundNormal="))
            .expect("BackgroundNormal line missing");
        let parts: Vec<u8> = line
            .trim_start_matches("BackgroundNormal=")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1], parts[2])
    }

    fn lum(rgb: (u8, u8, u8)) -> f32 {
        0.299 * rgb.0 as f32 + 0.587 * rgb.1 as f32 + 0.114 * rgb.2 as f32
    }

    #[test]
    fn light_kvantum_theme_produces_light_header_and_tooltip() {
        // Regression: before the fix, `[Colors:Header]` and tooltip bg were
        // mapped from `light.color`/`dark.color` — which in QPalette are 3D
        // shading roles, not surface colors. A light theme like Teto uses
        // `light.color=#22191b` (near-black) for bezel highlights, which
        // produced dark-on-dark sections.
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#fff8f8".to_string()); // light
        colors.set_value("window.text.color", "#22191b".to_string()); // dark
        colors.set_value("base.color", "#fff8f8".to_string());
        colors.set_value("text.color", "#22191b".to_string());
        colors.set_value("button.color", "#f2dde1".to_string());
        colors.set_value("button.text.color", "#22191b".to_string());
        colors.set_value("light.color", "#22191b".to_string()); // the trap
        colors.set_value("dark.color", "#837376".to_string());
        colors.set_value("highlight.color", "#8c4a5f".to_string());
        colors.set_value("highlight.text.color", "#ffffff".to_string());

        let content = build_plasma_scheme_from_kvantum("LightRegression", &colors);

        let window_lum = lum(section_bg_rgb(&content, "Colors:Window"));
        let header_lum = lum(section_bg_rgb(&content, "Colors:Header"));
        let tooltip_lum = lum(section_bg_rgb(&content, "Colors:Tooltip"));

        // Window is near-white (~253). Header/tooltip must stay bright —
        // i.e. a *darker* tint of window but nowhere near the (near-black)
        // `light.color`. Require luminance > 200.
        assert!(
            header_lum > 200.0,
            "Header bg should be a light tint of window (lum {})",
            header_lum
        );
        assert!(
            tooltip_lum > 200.0,
            "Tooltip bg should be a light tint of window (lum {})",
            tooltip_lum
        );
        // And they must be *slightly darker* than window (toward the text),
        // otherwise they'd be indistinguishable from the window background.
        assert!(header_lum < window_lum);
        assert!(tooltip_lum < window_lum);
    }

    #[test]
    fn dark_kvantum_theme_produces_dark_header_and_tooltip() {
        // Symmetric regression for dark themes — `toward_fg` should lift
        // the header/tooltip slightly TOWARD the (light) foreground, not
        // flip them to white. Models NoMansSkyJux.
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#101010".to_string()); // very dark
        colors.set_value("window.text.color", "#dcdcdc".to_string()); // light
        colors.set_value("base.color", "#101010".to_string());
        colors.set_value("text.color", "#dcdcdc".to_string());
        colors.set_value("button.color", "#1e1e20".to_string());
        colors.set_value("button.text.color", "#dcdcdc".to_string());
        // In a well-designed dark theme, `light.color` IS lighter than
        // button (it's the 3D bezel highlight). The old buggy mapping
        // happened to look OK here — but the fixed mapping still uses
        // `toward_fg(window, fg, t)` so we verify that instead.
        colors.set_value("light.color", "#464648".to_string());
        colors.set_value("dark.color", "#141416".to_string());
        colors.set_value("mid.color", "#38383a".to_string());
        colors.set_value("highlight.color", "#808080".to_string());
        colors.set_value("highlight.text.color", "#ffffff".to_string());
        colors.set_value("disabled.text.color", "#dcdcdc".to_string());

        let content = build_plasma_scheme_from_kvantum("DarkRegression", &colors);

        let window_lum = lum(section_bg_rgb(&content, "Colors:Window"));
        let header_lum = lum(section_bg_rgb(&content, "Colors:Header"));
        let tooltip_lum = lum(section_bg_rgb(&content, "Colors:Tooltip"));
        let complementary_lum = lum(section_bg_rgb(&content, "Colors:Complementary"));

        // Window is near-black (~16). Header/tooltip must remain dark
        // — certainly well below midpoint, definitely below 128.
        assert!(
            header_lum < 128.0,
            "Header bg should stay dark (lum {})",
            header_lum
        );
        assert!(
            tooltip_lum < 128.0,
            "Tooltip bg should stay dark (lum {})",
            tooltip_lum
        );

        // And they must be *brighter* than window (toward the fg), so they
        // read as distinct elevated surfaces rather than fusing into the bg.
        assert!(
            header_lum > window_lum,
            "Header bg ({}) should be brighter than window ({}) in dark theme",
            header_lum,
            window_lum
        );
        assert!(tooltip_lum > window_lum);
        assert!(complementary_lum > window_lum);
        // Elevation hierarchy: complementary (t=0.04) < header (t=0.08) < tooltip (t=0.15)
        assert!(complementary_lum < header_lum);
        assert!(header_lum < tooltip_lum);
    }

    #[test]
    fn kvantum_theme_with_alpha_channel_ignores_alpha() {
        // NoMansSkyJux uses `#RRGGBBAA` values like `#101010a0`. The Plasma
        // `.colors` format expects plain R,G,B — alpha must be stripped.
        let mut colors = KvantumColors::empty();
        colors.set_value("window.color", "#101010a0".to_string());
        colors.set_value("window.text.color", "#dcdcdc".to_string());
        colors.set_value("base.color", "#101010a0".to_string());
        colors.set_value("alt.base.color", "#1e1e20a0".to_string());
        colors.set_value("text.color", "#dcdcdc".to_string());
        colors.set_value("button.color", "#1e1e20".to_string());
        colors.set_value("button.text.color", "#dcdcdc".to_string());
        colors.set_value("highlight.color", "#808080".to_string());
        colors.set_value("highlight.text.color", "#ffffff".to_string());

        let content = build_plasma_scheme_from_kvantum("AlphaTest", &colors);

        // BackgroundNormal lines must be exactly 3 comma-separated ints.
        for line in content.lines().filter(|l| l.starts_with("BackgroundNormal=")) {
            let v = line.trim_start_matches("BackgroundNormal=");
            let parts: Vec<&str> = v.split(',').collect();
            assert_eq!(
                parts.len(),
                3,
                "BackgroundNormal must be R,G,B (no alpha) — got '{}'",
                v
            );
            for p in parts {
                p.parse::<u8>()
                    .unwrap_or_else(|_| panic!("non-u8 component: {}", p));
            }
        }
        assert!(content.contains("BackgroundNormal=16,16,16"));
    }
}
