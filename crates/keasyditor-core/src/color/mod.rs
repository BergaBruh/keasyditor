use std::fmt;

/// RGBA color with 8-bit components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(
                f,
                "#{:02x}{:02x}{:02x}{:02x}",
                self.r, self.g, self.b, self.a
            )
        }
    }
}

/// Parse Klassy "R,G,B" format (e.g. "255,179,174").
pub fn parse_klassy_color(s: &str) -> Option<Rgba> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;
    Some(Rgba::rgb(r, g, b))
}

/// Format as Klassy "R,G,B".
pub fn to_klassy_color(c: &Rgba) -> String {
    format!("{},{},{}", c.r, c.g, c.b)
}

/// Parse Kvantum "#RRGGBB" or "#RRGGBBAA" format.
pub fn parse_kvantum_color(s: &str) -> Option<Rgba> {
    let s = s.strip_prefix('#')?;
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(Rgba::rgb(r, g, b))
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some(Rgba::new(r, g, b, a))
        }
        _ => None,
    }
}

/// Format as Kvantum "#RRGGBB" (or "#RRGGBBAA" if alpha < 255).
pub fn to_kvantum_color(c: &Rgba) -> String {
    format!("{}", c)
}

/// Named color lookup.
fn named_color(name: &str) -> Option<Rgba> {
    match name.to_lowercase().as_str() {
        "white" => Some(Rgba::rgb(255, 255, 255)),
        "black" => Some(Rgba::rgb(0, 0, 0)),
        "red" => Some(Rgba::rgb(255, 0, 0)),
        "green" => Some(Rgba::rgb(0, 128, 0)),
        "blue" => Some(Rgba::rgb(0, 0, 255)),
        "yellow" => Some(Rgba::rgb(255, 255, 0)),
        "cyan" => Some(Rgba::rgb(0, 255, 255)),
        "magenta" => Some(Rgba::rgb(255, 0, 255)),
        "transparent" => Some(Rgba::new(0, 0, 0, 0)),
        _ => None,
    }
}

/// Try to parse a color from any supported format (hex, R,G,B, named).
pub fn try_parse(s: &str) -> Option<Rgba> {
    let s = s.trim();
    if s.starts_with('#') {
        parse_kvantum_color(s)
    } else if s.contains(',') {
        parse_klassy_color(s)
    } else {
        named_color(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn klassy_round_trip() {
        let c = Rgba::rgb(255, 179, 174);
        assert_eq!(to_klassy_color(&c), "255,179,174");
        assert_eq!(parse_klassy_color("255,179,174"), Some(c));
    }

    #[test]
    fn kvantum_hex6() {
        let c = parse_kvantum_color("#1a1111").unwrap();
        assert_eq!(c, Rgba::rgb(0x1a, 0x11, 0x11));
    }

    #[test]
    fn kvantum_hex8() {
        let c = parse_kvantum_color("#1a111180").unwrap();
        assert_eq!(c, Rgba::new(0x1a, 0x11, 0x11, 0x80));
    }

    #[test]
    fn named_colors() {
        assert_eq!(try_parse("white"), Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(try_parse("black"), Some(Rgba::rgb(0, 0, 0)));
        assert_eq!(try_parse("transparent"), Some(Rgba::new(0, 0, 0, 0)));
    }

    #[test]
    fn try_parse_all_formats() {
        assert!(try_parse("#ff0000").is_some());
        assert!(try_parse("255,0,0").is_some());
        assert!(try_parse("red").is_some());
        assert!(try_parse("nonsense").is_none());
    }
}
