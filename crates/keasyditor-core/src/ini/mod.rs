mod parser;
mod serializer;

pub use parser::parse_ini;
pub use serializer::serialize_ini;

/// A single key=value entry, optionally with an inline comment.
#[derive(Debug, Clone, PartialEq)]
pub struct IniEntry {
    pub key: String,
    pub value: String,
    /// Inline comment (after value), if any.
    pub comment: Option<String>,
}

/// A named INI section with its entries and surrounding comments/blank lines.
#[derive(Debug, Clone, PartialEq)]
pub struct IniSection {
    /// Section name (e.g. "Windeco", "%General").
    pub name: String,
    /// Key-value entries in order.
    pub entries: Vec<IniEntry>,
    /// Comment/blank lines preceding the `[SectionName]` header.
    pub preceding_lines: Vec<String>,
}

/// A full INI document preserving comments and ordering for round-trip fidelity.
#[derive(Debug, Clone, PartialEq)]
pub struct IniDocument {
    /// Comment/blank lines before the first section header.
    pub header_lines: Vec<String>,
    /// Sections in file order.
    pub sections: Vec<IniSection>,
    /// Trailing lines after the last section.
    pub trailing_lines: Vec<String>,
}

impl IniDocument {
    /// Look up a value by section name and key.
    pub fn get_value(&self, section: &str, key: &str) -> Option<&str> {
        self.sections
            .iter()
            .find(|s| s.name == section)
            .and_then(|s| {
                s.entries
                    .iter()
                    .find(|e| e.key == key)
                    .map(|e| e.value.as_str())
            })
    }

    /// Set a value in a section. Creates the section/entry if it doesn't exist.
    pub fn set_value(&mut self, section: &str, key: &str, value: String) {
        let sec = match self.sections.iter_mut().find(|s| s.name == section) {
            Some(s) => s,
            None => {
                self.sections.push(IniSection {
                    name: section.to_string(),
                    entries: Vec::new(),
                    preceding_lines: vec![String::new()],
                });
                self.sections.last_mut().unwrap()
            }
        };
        match sec.entries.iter_mut().find(|e| e.key == key) {
            Some(e) => e.value = value,
            None => sec.entries.push(IniEntry {
                key: key.to_string(),
                value,
                comment: None,
            }),
        }
    }

    /// Remove a key from a section. Returns true if found and removed.
    pub fn remove_value(&mut self, section: &str, key: &str) -> bool {
        if let Some(sec) = self.sections.iter_mut().find(|s| s.name == section) {
            let before = sec.entries.len();
            sec.entries.retain(|e| e.key != key);
            sec.entries.len() < before
        } else {
            false
        }
    }

    /// Get all section names.
    pub fn section_names(&self) -> Vec<&str> {
        self.sections.iter().map(|s| s.name.as_str()).collect()
    }
}
