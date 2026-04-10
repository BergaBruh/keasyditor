use super::IniDocument;

/// Serialize an `IniDocument` back to a string, preserving comments, blank lines,
/// and ordering for round-trip fidelity.
pub fn serialize_ini(doc: &IniDocument) -> String {
    let mut out = String::new();

    // Header lines (before first section)
    for line in &doc.header_lines {
        out.push_str(line);
        out.push('\n');
    }

    for section in &doc.sections {
        // Preceding lines (comments/blanks before section header)
        for line in &section.preceding_lines {
            out.push_str(line);
            out.push('\n');
        }

        // Section header
        out.push('[');
        out.push_str(&section.name);
        out.push(']');
        out.push('\n');

        // Entries
        for entry in &section.entries {
            if entry.key.is_empty() {
                // This is a preserved comment/blank line within the section
                if let Some(ref comment) = entry.comment {
                    out.push_str(comment);
                }
                out.push('\n');
            } else {
                out.push_str(&entry.key);
                out.push('=');
                out.push_str(&entry.value);
                if let Some(ref comment) = entry.comment {
                    out.push(' ');
                    out.push_str(comment);
                }
                out.push('\n');
            }
        }
    }

    // Trailing lines
    for line in &doc.trailing_lines {
        out.push_str(line);
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::super::parser::parse_ini;
    use super::*;

    #[test]
    fn round_trip_simple() {
        let input = "[Section]\nkey=value\n";
        let doc = parse_ini(input);
        let output = serialize_ini(&doc);
        assert_eq!(output, input);
    }

    #[test]
    fn round_trip_with_comments() {
        let input = "# Top comment\n[Section]\nkey=value\n";
        let doc = parse_ini(input);
        let output = serialize_ini(&doc);
        assert_eq!(output, input);
    }

    #[test]
    fn round_trip_multiple_sections() {
        let input = "[A]\na=1\n\n[B]\nb=2\n";
        let doc = parse_ini(input);
        let output = serialize_ini(&doc);
        assert_eq!(output, input);
    }
}
