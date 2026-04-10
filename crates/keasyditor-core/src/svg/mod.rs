/// SVG XML manipulation helpers for Kvantum theme SVG files.
///
/// Uses `quick-xml` for reading/parsing and string manipulation for writing.
/// This approach is simpler and avoids quick-xml writer lifetime complexities.
use std::collections::HashMap;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Parse CSS style string into a property map.
fn parse_style(style: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in style.split(';') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(colon_idx) = trimmed.find(':') {
            let key = trimmed[..colon_idx].trim().to_string();
            let value = trimmed[colon_idx + 1..].trim().to_string();
            map.insert(key, value);
        }
    }
    map
}

/// Serialize a property map back into a CSS style string.
fn serialize_style(props: &HashMap<String, String>) -> String {
    let mut parts: Vec<String> = props
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();
    parts.sort(); // deterministic output
    parts.join(";")
}

/// Information about an element found by ID.
struct ElementInfo {
    /// Direct attributes as key-value pairs.
    attrs: HashMap<String, String>,
}

/// Find an element by ID and return its attributes.
fn find_element_attrs(svg_content: &str, element_id: &str) -> Option<ElementInfo> {
    let mut reader = Reader::from_str(svg_content);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref tag)) | Ok(Event::Empty(ref tag)) => {
                let mut attrs = HashMap::new();
                let mut found_id = false;
                for attr in tag.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = String::from_utf8_lossy(&attr.value).to_string();
                    if key == "id" && val == element_id {
                        found_id = true;
                    }
                    attrs.insert(key, val);
                }
                if found_id {
                    return Some(ElementInfo { attrs });
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    None
}

/// Get the fill color from an SVG element.
///
/// Checks the `fill` attribute first, then falls back to the `style`
/// attribute (CSS `fill:` property).
pub fn get_fill_color(svg_content: &str, element_id: &str) -> Option<String> {
    get_element_color_attr(svg_content, element_id, "fill")
}

/// Get the stroke color from an SVG element.
pub fn get_stroke_color(svg_content: &str, element_id: &str) -> Option<String> {
    get_element_color_attr(svg_content, element_id, "stroke")
}

/// Get a color attribute (fill or stroke) from an element.
fn get_element_color_attr(svg_content: &str, element_id: &str, attr_name: &str) -> Option<String> {
    let info = find_element_attrs(svg_content, element_id)?;

    // Check direct attribute first
    if let Some(val) = info.attrs.get(attr_name) {
        if !val.is_empty() {
            return Some(val.clone());
        }
    }

    // Fall back to style attribute
    if let Some(style) = info.attrs.get("style") {
        let props = parse_style(style);
        if let Some(val) = props.get(attr_name) {
            return Some(val.clone());
        }
    }

    None
}

/// Set the fill color on an element in the SVG content.
///
/// Returns the modified SVG content, or the original if the element was not found.
pub fn set_fill_color(svg_content: &str, element_id: &str, color: &str) -> String {
    set_element_color_attr(svg_content, element_id, "fill", color)
}

/// Set the stroke color on an element in the SVG content.
pub fn set_stroke_color(svg_content: &str, element_id: &str, color: &str) -> String {
    set_element_color_attr(svg_content, element_id, "stroke", color)
}

/// Set a color attribute on an element using string-based replacement.
///
/// This approach avoids the complexity of a full XML round-trip while being
/// reliable for the structured SVG files used by Kvantum themes.
fn set_element_color_attr(
    svg_content: &str,
    element_id: &str,
    attr_name: &str,
    color: &str,
) -> String {
    // First, check if the element uses style or direct attribute
    let info = match find_element_attrs(svg_content, element_id) {
        Some(info) => info,
        None => return svg_content.to_string(),
    };

    let has_style_prop = if let Some(style) = info.attrs.get("style") {
        let props = parse_style(style);
        props.contains_key(attr_name)
    } else {
        false
    };

    // Find the element tag in the raw SVG and modify it
    // We search for the tag containing id="element_id"
    let id_pattern = format!("id=\"{}\"", element_id);
    let result = svg_content.to_string();

    // Find the position of the id attribute
    if let Some(id_pos) = result.find(&id_pattern) {
        // Find the opening < before this id
        let tag_start = match result[..id_pos].rfind('<') {
            Some(pos) => pos,
            None => return svg_content.to_string(),
        };
        // Find the closing > or /> after the id
        let after_id = id_pos + id_pattern.len();
        let tag_end = match result[after_id..].find('>') {
            Some(pos) => pos + after_id,
            None => return svg_content.to_string(),
        };

        let tag_content = &result[tag_start..=tag_end];

        let new_tag = if has_style_prop {
            // Modify the style attribute
            if let Some(style_val) = info.attrs.get("style") {
                let mut props = parse_style(style_val);
                props.insert(attr_name.to_string(), color.to_string());
                let new_style = serialize_style(&props);
                let old_style_attr = format!("style=\"{}\"", style_val);
                let new_style_attr = format!("style=\"{}\"", new_style);
                tag_content.replace(&old_style_attr, &new_style_attr)
            } else {
                tag_content.to_string()
            }
        } else if info.attrs.contains_key(attr_name) {
            // Replace existing direct attribute
            let old_val = &info.attrs[attr_name];
            let old_attr = format!("{}=\"{}\"", attr_name, old_val);
            let new_attr = format!("{}=\"{}\"", attr_name, color);
            tag_content.replace(&old_attr, &new_attr)
        } else {
            // Add the attribute after the id attribute
            let new_attr = format!(" {}=\"{}\"", attr_name, color);
            tag_content.replace(&id_pattern, &format!("{}{}", id_pattern, new_attr))
        };

        let mut result = result.to_string();
        result.replace_range(tag_start..=tag_end, &new_tag);
        return result;
    }

    svg_content.to_string()
}

/// Return all element IDs found in the SVG content.
pub fn get_all_element_ids(svg_content: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut reader = Reader::from_str(svg_content);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref tag)) | Ok(Event::Empty(ref tag)) => {
                for attr in tag.attributes().flatten() {
                    if attr.key.as_ref() == b"id" {
                        if let Ok(val) = String::from_utf8(attr.value.to_vec()) {
                            if !val.is_empty() {
                                ids.push(val);
                            }
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    ids
}

/// Position suffixes that should be stripped when grouping by widget type.
const POSITION_SUFFIXES: &[&str] = &[
    "-bottomright",
    "-bottomleft",
    "-topright",
    "-topleft",
    "-bottom",
    "-right",
    "-left",
    "-top",
];

/// Known multi-word widget prefixes (order matters: longest first).
const COMPOUND_PREFIXES: &[&str] = &[
    "flat-button",
    "expand-button",
    "border-button",
    "item-button",
    "menu-item",
    "menu-indicator",
    "menu-separator",
    "mdi-button",
    "mdi-close",
    "mdi-maximize",
    "mdi-minimize",
    "mdi-restore",
    "dial-handle",
];

/// Catalog all named elements in the SVG grouped by base widget type.
///
/// Returns a map where each key is a widget type and the value is a sorted
/// list of unique states.
pub fn catalog_elements(svg_content: &str) -> HashMap<String, Vec<String>> {
    let ids = get_all_element_ids(svg_content);
    let mut catalog: HashMap<String, std::collections::BTreeSet<String>> = HashMap::new();

    for id in ids {
        // Strip position suffixes
        let mut normalized = id.clone();
        for suffix in POSITION_SUFFIXES {
            if let Some(stripped) = normalized.strip_suffix(suffix) {
                normalized = stripped.to_string();
                break;
            }
        }

        // Split into widget type and state
        if let Some((widget_type, state)) = split_widget_state(&normalized) {
            catalog.entry(widget_type).or_default().insert(state);
        }
    }

    catalog
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()
}

/// Split an element ID (after position stripping) into (widget_type, state).
fn split_widget_state(id: &str) -> Option<(String, String)> {
    // Try compound prefixes first
    for prefix in COMPOUND_PREFIXES {
        let with_dash = format!("{}-", prefix);
        if id.starts_with(&with_dash) && id.len() > with_dash.len() {
            return Some((prefix.to_string(), id[with_dash.len()..].to_string()));
        }
        if id == *prefix {
            return Some((prefix.to_string(), "default".to_string()));
        }
    }

    // Generic split: everything up to the last `-` is the widget type
    match id.rfind('-') {
        Some(pos) if pos > 0 && pos < id.len() - 1 => {
            Some((id[..pos].to_string(), id[pos + 1..].to_string()))
        }
        _ => Some((id.to_string(), "default".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_SVG: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
  <rect id="button-normal" fill="#ff0000" width="50" height="50"/>
  <rect id="button-pressed" fill="#00ff00" width="50" height="50"/>
  <rect id="checkbox-normal" style="fill:#0000ff;stroke:#000000" width="20" height="20"/>
  <g id="group1">
    <rect id="nested-item" fill="#aabbcc" width="10" height="10"/>
  </g>
</svg>"##;

    #[test]
    fn get_all_ids() {
        let ids = get_all_element_ids(SIMPLE_SVG);
        assert!(ids.contains(&"button-normal".to_string()));
        assert!(ids.contains(&"button-pressed".to_string()));
        assert!(ids.contains(&"checkbox-normal".to_string()));
        assert!(ids.contains(&"group1".to_string()));
        assert!(ids.contains(&"nested-item".to_string()));
    }

    #[test]
    fn get_fill_from_attribute() {
        assert_eq!(
            get_fill_color(SIMPLE_SVG, "button-normal"),
            Some("#ff0000".to_string())
        );
        assert_eq!(
            get_fill_color(SIMPLE_SVG, "button-pressed"),
            Some("#00ff00".to_string())
        );
    }

    #[test]
    fn get_fill_from_style() {
        assert_eq!(
            get_fill_color(SIMPLE_SVG, "checkbox-normal"),
            Some("#0000ff".to_string())
        );
    }

    #[test]
    fn get_stroke_from_style() {
        assert_eq!(
            get_stroke_color(SIMPLE_SVG, "checkbox-normal"),
            Some("#000000".to_string())
        );
    }

    #[test]
    fn get_nonexistent_element() {
        assert_eq!(get_fill_color(SIMPLE_SVG, "nonexistent"), None);
    }

    #[test]
    fn set_fill_on_attribute() {
        let result = set_fill_color(SIMPLE_SVG, "button-normal", "#ffffff");
        assert_eq!(
            get_fill_color(&result, "button-normal"),
            Some("#ffffff".to_string())
        );
        // Other elements unchanged
        assert_eq!(
            get_fill_color(&result, "button-pressed"),
            Some("#00ff00".to_string())
        );
    }

    #[test]
    fn set_fill_on_style() {
        let result = set_fill_color(SIMPLE_SVG, "checkbox-normal", "#112233");
        assert_eq!(
            get_fill_color(&result, "checkbox-normal"),
            Some("#112233".to_string())
        );
        // Stroke should be preserved
        assert_eq!(
            get_stroke_color(&result, "checkbox-normal"),
            Some("#000000".to_string())
        );
    }

    #[test]
    fn catalog_basic() {
        let catalog = catalog_elements(SIMPLE_SVG);
        assert!(catalog.contains_key("button"));
        let button_states = catalog.get("button").unwrap();
        assert!(button_states.contains(&"normal".to_string()));
        assert!(button_states.contains(&"pressed".to_string()));

        assert!(catalog.contains_key("checkbox"));
        let checkbox_states = catalog.get("checkbox").unwrap();
        assert!(checkbox_states.contains(&"normal".to_string()));
    }

    #[test]
    fn catalog_with_positions() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <rect id="button-normal-topleft"/>
  <rect id="button-normal-topright"/>
  <rect id="button-normal-bottomleft"/>
  <rect id="button-pressed"/>
</svg>"#;
        let catalog = catalog_elements(svg);
        let states = catalog.get("button").unwrap();
        assert!(states.contains(&"normal".to_string()));
        assert!(states.contains(&"pressed".to_string()));
    }

    #[test]
    fn catalog_compound_prefixes() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <rect id="flat-button-normal"/>
  <rect id="flat-button-pressed"/>
  <rect id="menu-item-normal"/>
</svg>"#;
        let catalog = catalog_elements(svg);
        assert!(catalog.contains_key("flat-button"));
        assert!(catalog.contains_key("menu-item"));
    }

    #[test]
    fn parse_style_helper() {
        let style = "fill:#ff0000;stroke:#000000;opacity:0.5";
        let props = parse_style(style);
        assert_eq!(props.get("fill"), Some(&"#ff0000".to_string()));
        assert_eq!(props.get("stroke"), Some(&"#000000".to_string()));
        assert_eq!(props.get("opacity"), Some(&"0.5".to_string()));
    }

    #[test]
    fn split_widget_state_basic() {
        assert_eq!(
            split_widget_state("button-normal"),
            Some(("button".to_string(), "normal".to_string()))
        );
        assert_eq!(
            split_widget_state("flat-button-pressed"),
            Some(("flat-button".to_string(), "pressed".to_string()))
        );
        assert_eq!(
            split_widget_state("standalone"),
            Some(("standalone".to_string(), "default".to_string()))
        );
    }
}
