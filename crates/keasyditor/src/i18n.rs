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
    let candidates = detect_locales();
    let map = build_map(&candidates);
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

/// Return a priority-ordered list of `(full_locale, lang)` candidates
/// collected from the standard gettext environment variables.
///
/// `LANGUAGE` is treated as a colon-separated list per gettext convention
/// (e.g. `LANGUAGE=ru:en_US` → two candidates: `("ru","ru")`, `("en_US","en")`).
/// All other vars contribute a single candidate each. Empty / `C` / `POSIX`
/// values are skipped. Encoding and modifier suffixes (`.UTF-8`, `@euro`)
/// are stripped.
fn detect_locales() -> Vec<(String, String)> {
    let values: Vec<Option<String>> = ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .map(|v| std::env::var(v).ok())
        .collect();
    parse_locale_env(&values)
}

/// Pure form of [`detect_locales`]: takes raw env values (in the order
/// `LANGUAGE, LC_ALL, LC_MESSAGES, LANG`) and produces candidate pairs.
/// Extracted so it can be unit-tested without touching process env.
fn parse_locale_env(values: &[Option<String>]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut push = |locale: String| {
        if locale.is_empty() {
            return;
        }
        let lang = locale.split('_').next().unwrap_or("").to_string();
        let entry = (locale, lang);
        if !out.contains(&entry) {
            out.push(entry);
        }
    };
    for val in values.iter().flatten() {
        let val = val.trim();
        if val.is_empty() || val == "C" || val == "POSIX" {
            continue;
        }
        for piece in val.split(':') {
            let piece = piece.trim();
            if piece.is_empty() || piece == "C" || piece == "POSIX" {
                continue;
            }
            // Strip encoding/modifier: "ru_RU.UTF-8@euro" → "ru_RU"
            let locale = piece
                .split(['.', '@'])
                .next()
                .unwrap_or("")
                .to_string();
            push(locale);
        }
    }
    if out.is_empty() {
        out.push(("en".to_string(), "en".to_string()));
    }
    out
}

// ── Map construction ──────────────────────────────────────────────────────────

/// Build the final translation map:
/// 1. Start with embedded English (so every key has a value).
/// 2. Walk the candidate list in priority order and overlay the first
///    match. For each candidate, try the exact locale file
///    (`ru_RU.toml`), then the language-only file (`ru.toml`), then any
///    region variant of that language (`ru_*.toml`). The region-variant
///    pass matters because `LANGUAGE=ru` yields `("ru","ru")` but we
///    only ship `ru_RU.toml`.
fn build_map(candidates: &[(String, String)]) -> HashMap<String, String> {
    let mut map = parse(EN_EMBEDDED);

    for (full_locale, lang) in candidates {
        // English is the embedded base — no file to load.
        if full_locale == "en" || full_locale.starts_with("en_") {
            return map;
        }
        let loaded = read_locale_file(full_locale)
            .or_else(|| {
                if lang != full_locale {
                    read_locale_file(lang)
                } else {
                    None
                }
            })
            .or_else(|| read_region_variant(lang));
        if let Some(content) = loaded {
            for (k, v) in parse(&content) {
                map.insert(k, v);
            }
            return map;
        }
    }

    map
}

/// Find any `<lang>_<REGION>.toml` in the search path and return its
/// content. Used as a last-resort fallback when the environment only
/// gives us a language code (e.g. `LANGUAGE=ru`) but we ship locales
/// keyed by region (e.g. `ru_RU.toml`).
fn read_region_variant(lang: &str) -> Option<String> {
    if lang.is_empty() {
        return None;
    }
    let prefix = format!("{}_", lang);
    for dir in search_dirs() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            if name.starts_with(&prefix) && name.ends_with(".toml")
                && let Ok(content) = std::fs::read_to_string(entry.path())
            {
                return Some(content);
            }
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn parses_gettext_language_list() {
        // LANGUAGE=ru:en_US — a gettext priority list, not a single locale.
        let got = parse_locale_env(&[s("ru:en_US"), None, None, s("ru_RU.UTF-8")]);
        assert_eq!(
            got,
            vec![
                ("ru".to_string(), "ru".to_string()),
                ("en_US".to_string(), "en".to_string()),
                ("ru_RU".to_string(), "ru".to_string()),
            ]
        );
    }

    #[test]
    fn strips_encoding_and_modifier() {
        let got = parse_locale_env(&[None, None, None, s("de_DE.UTF-8@euro")]);
        assert_eq!(got, vec![("de_DE".to_string(), "de".to_string())]);
    }

    #[test]
    fn skips_posix_and_c() {
        let got = parse_locale_env(&[s("C"), s("POSIX"), None, None]);
        assert_eq!(got, vec![("en".to_string(), "en".to_string())]);
    }

    #[test]
    fn defaults_to_english_when_empty() {
        let got = parse_locale_env(&[None, None, None, None]);
        assert_eq!(got, vec![("en".to_string(), "en".to_string())]);
    }

    #[test]
    fn dedupes_repeated_locales() {
        let got = parse_locale_env(&[s("ru_RU"), None, None, s("ru_RU.UTF-8")]);
        assert_eq!(got, vec![("ru_RU".to_string(), "ru".to_string())]);
    }
}
