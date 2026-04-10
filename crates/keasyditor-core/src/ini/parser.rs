use super::{IniDocument, IniEntry, IniSection};

/// Parse an INI string into an `IniDocument`, preserving comments, blank lines,
/// and section/key ordering for round-trip fidelity.
///
/// Handles Kvantum quirks: `[%General]` section names, dotted keys like
/// `text.focus.color`, and inline `#` comments.
pub fn parse_ini(input: &str) -> IniDocument {
    let mut header_lines: Vec<String> = Vec::new();
    let mut sections: Vec<IniSection> = Vec::new();
    let mut pending_lines: Vec<String> = Vec::new();
    let mut current_section: Option<IniSection> = None;

    for line in input.lines() {
        let trimmed = line.trim();

        // Section header: [Name] or [%General]
        if trimmed.starts_with('[')
            && let Some(end) = trimmed.find(']') {
                let name = trimmed[1..end].to_string();

                // Push previous section if any
                if let Some(sec) = current_section.take() {
                    sections.push(sec);
                }

                current_section = Some(IniSection {
                    name,
                    entries: Vec::new(),
                    preceding_lines: std::mem::take(&mut pending_lines),
                });
                continue;
            }

        // If no section yet, these are header lines
        if current_section.is_none() {
            if sections.is_empty() {
                header_lines.push(line.to_string());
            } else {
                pending_lines.push(line.to_string());
            }
            continue;
        }

        // Empty line or comment line
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            // Could be inter-entry comment; store as a "comment entry"
            // For simplicity, we treat blank/comment lines between entries
            // as pending lines for the next section OR trailing lines.
            // But to preserve them within a section, let's use a sentinel approach:
            // We'll push them as pending for the next section header.
            // Actually, for proper round-trip, let's check if more entries follow.
            // Simpler: just accumulate in pending_lines. If another key=value comes,
            // they get re-attached. If a section header comes, they become preceding_lines.
            pending_lines.push(line.to_string());
            continue;
        }

        // Key=value line
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let rest = trimmed[eq_pos + 1..].trim();

            // Detect inline comment: look for ` #` or ` ;` pattern
            // But be careful not to split hex colors like #RRGGBB
            let (value, comment) = split_inline_comment(rest);

            if let Some(sec) = current_section.as_mut() {
                // Re-attach any pending blank/comment lines as preceding comments
                // within the section. We store them as comment-only entries.
                for pl in pending_lines.drain(..) {
                    sec.entries.push(IniEntry {
                        key: String::new(),
                        value: String::new(),
                        comment: Some(pl),
                    });
                }
                sec.entries.push(IniEntry {
                    key,
                    value: value.to_string(),
                    comment: comment.map(|s| s.to_string()),
                });
            }
        } else {
            // Unrecognized line — preserve as-is
            pending_lines.push(line.to_string());
        }
    }

    // Push last section
    if let Some(sec) = current_section.take() {
        sections.push(sec);
    }

    IniDocument {
        header_lines,
        sections,
        trailing_lines: pending_lines,
    }
}

/// Split a value string from a potential inline comment.
/// Returns (value, optional_comment).
///
/// Heuristic: a `#` preceded by whitespace and NOT part of a hex color is a comment.
fn split_inline_comment(rest: &str) -> (&str, Option<&str>) {
    // Look for ` #` pattern that's likely a comment (not inside a hex value)
    // Simple heuristic: find ` #` where the # is followed by a space or letter
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' && i > 0 && bytes[i - 1] == b' ' {
            // Check it's not a hex color: hex colors are like #RRGGBB (6 or 8 hex digits)
            let after = &rest[i + 1..];
            let hex_len = after.bytes().take_while(|b| b.is_ascii_hexdigit()).count();
            if hex_len != 6 && hex_len != 8 {
                let value = rest[..i].trim_end();
                let comment = rest[i..].trim();
                return (value, Some(comment));
            }
        }
        i += 1;
    }
    (rest, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_ini() {
        let input = "[Section]\nkey=value\n";
        let doc = parse_ini(input);
        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.sections[0].name, "Section");
        assert_eq!(doc.get_value("Section", "key"), Some("value"));
    }

    #[test]
    fn parse_kvantum_percent_general() {
        let input = "[%General]\ncomposite=true\n";
        let doc = parse_ini(input);
        assert_eq!(doc.sections[0].name, "%General");
        assert_eq!(doc.get_value("%General", "composite"), Some("true"));
    }

    #[test]
    fn parse_dotted_keys() {
        let input = "[GeneralColors]\nwindow.color=#1a1111\ntext.focus.color=#e05555\n";
        let doc = parse_ini(input);
        assert_eq!(
            doc.get_value("GeneralColors", "window.color"),
            Some("#1a1111")
        );
        assert_eq!(
            doc.get_value("GeneralColors", "text.focus.color"),
            Some("#e05555")
        );
    }

    #[test]
    fn preserve_comments() {
        let input = "# Header comment\n[Section]\n# Entry comment\nkey=value\n";
        let doc = parse_ini(input);
        assert_eq!(doc.header_lines.len(), 1);
        assert_eq!(doc.header_lines[0], "# Header comment");
    }

    #[test]
    fn set_and_get_value() {
        let mut doc = parse_ini("[S]\na=1\n");
        doc.set_value("S", "a", "2".to_string());
        assert_eq!(doc.get_value("S", "a"), Some("2"));
        doc.set_value("S", "b", "3".to_string());
        assert_eq!(doc.get_value("S", "b"), Some("3"));
    }

    #[test]
    fn remove_value() {
        let mut doc = parse_ini("[S]\na=1\nb=2\n");
        assert!(doc.remove_value("S", "a"));
        assert_eq!(doc.get_value("S", "a"), None);
        assert_eq!(doc.get_value("S", "b"), Some("2"));
    }
}
