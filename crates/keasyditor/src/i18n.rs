use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

static LOCALE: OnceLock<HashMap<String, String>> = OnceLock::new();

/// English is always embedded — the binary works without any locale files on disk.
const EN_EMBEDDED: &str = include_str!("locale/en.toml");

/// During `cargo run` the source locale/ directory is known at compile time.
const DEV_LOCALE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/locale");

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialize locale. Call once at the start of main(), before iced starts.
pub fn init() {
    let (full_locale, lang) = detect_locale();
    let map = build_map(&full_locale, &lang);
    LOCALE.set(map).ok();
}

/// Translate a key. Falls back to English, then to the key itself.
pub fn t(key: &str) -> String {
    LOCALE
        .get()
        .and_then(|m| m.get(key))
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

// ── Locale detection ──────────────────────────────────────────────────────────

/// Returns (full_locale, lang):  "ru_RU.UTF-8" → ("ru_RU", "ru")
fn detect_locale() -> (String, String) {
    for var in ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(val) = std::env::var(var) {
            let val = val.trim().to_string();
            if !val.is_empty() && val != "C" && val != "POSIX" {
                // Strip encoding/modifier: "ru_RU.UTF-8@euro" → "ru_RU"
                let locale = val
                    .split(['.', '@'])
                    .next()
                    .unwrap_or("en")
                    .to_string();
                // Language code: "ru_RU" → "ru"
                let lang = locale.split('_').next().unwrap_or("en").to_string();
                if !locale.is_empty() {
                    return (locale, lang);
                }
            }
        }
    }
    ("en".to_string(), "en".to_string())
}

// ── Map construction ──────────────────────────────────────────────────────────

/// Build the final translation map:
/// 1. Start with embedded English (so every key has a value).
/// 2. Overlay the best matching locale file (exact → lang-only).
fn build_map(full_locale: &str, lang: &str) -> HashMap<String, String> {
    let mut map = parse(EN_EMBEDDED);

    // English is already loaded — nothing more to do.
    if full_locale == "en" || full_locale.starts_with("en_") {
        return map;
    }

    // Try exact locale (e.g. "ru_RU"), then language-only ("ru").
    let loaded = read_locale_file(full_locale)
        .or_else(|| if lang != full_locale { read_locale_file(lang) } else { None });

    if let Some(content) = loaded {
        // Overlay translated keys on top of the English base.
        for (k, v) in parse(&content) {
            map.insert(k, v);
        }
    }

    map
}

// ── File loading ──────────────────────────────────────────────────────────────

/// Look for `<name>.toml` in all search directories and return the first match.
fn read_locale_file(name: &str) -> Option<String> {
    let filename = format!("{}.toml", name);
    for dir in search_dirs() {
        let path = dir.join(&filename);
        if let Ok(content) = std::fs::read_to_string(&path) {
            return Some(content);
        }
    }
    None
}

/// Directories searched in priority order.
///
/// To add a new language: create `<locale>.toml` in any of these directories
/// (no code changes required).
fn search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    // 1. Explicit override — useful for testing.
    if let Ok(dir) = std::env::var("KEASYDITOR_LOCALE_DIR") {
        dirs.push(PathBuf::from(dir));
    }

    // 2. User config: ~/.config/keasyditor/locale/
    if let Some(cfg) = xdg_config_dir() {
        dirs.push(cfg.join("keasyditor").join("locale"));
    }

    // 3. Next to the executable: <exe_dir>/locale/
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("locale"));
        }

    // 4. System install: /usr/share/keasyditor/locale/
    dirs.push(PathBuf::from("/usr/share/keasyditor/locale"));

    // 5. Source tree — active during `cargo run` / `cargo build` in development.
    dirs.push(PathBuf::from(DEV_LOCALE_DIR));

    dirs
}

fn xdg_config_dir() -> Option<PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(".config"))
        })
}

// ── Parser ────────────────────────────────────────────────────────────────────

fn parse(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let k = k.trim().to_string();
            let v = v.trim();
            let v = if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
                &v[1..v.len() - 1]
            } else {
                v
            };
            if !k.is_empty() {
                map.insert(k, v.to_string());
            }
        }
    }
    map
}
