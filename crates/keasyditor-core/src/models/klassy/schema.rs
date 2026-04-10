/// Schema metadata for every known Klassy preset configuration key.
///
/// Provides type information, default values, min/max constraints, and enum
/// value lists so that a UI can render appropriate editors for each field.
use std::collections::HashMap;
use std::sync::LazyLock;

/// The data type of a configuration field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
    /// `true` / `false`
    Boolean,
    /// Whole number (e.g. `8`, `160`)
    Integer,
    /// Floating-point number (e.g. `4.5`)
    Double,
    /// Free-form string
    String,
    /// Klassy-format RGB color (`R,G,B`)
    Color,
    /// One of a fixed set of string constants
    EnumType,
}

/// Describes a single configuration key's type, default, and constraints.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldSchema {
    /// The INI key name (e.g. `WindowCornerRadius`).
    pub key: &'static str,
    /// The data type of this field.
    pub field_type: FieldType,
    /// Default value as a string.
    pub default_value: &'static str,
    /// Minimum allowed value for integer fields.
    pub min_int: Option<i64>,
    /// Maximum allowed value for integer fields.
    pub max_int: Option<i64>,
    /// Minimum allowed value for double fields.
    pub min_double: Option<f64>,
    /// Maximum allowed value for double fields.
    pub max_double: Option<f64>,
    /// Allowed string values when field_type is EnumType.
    pub enum_values: Option<&'static [&'static str]>,
    /// Human-readable description of the field.
    pub description: &'static str,
}

/// Static registry of all known Klassy preset configuration keys.
pub struct KlassySchema;

impl KlassySchema {
    /// All preset field schemas, keyed by INI key name.
    pub fn preset_fields() -> &'static HashMap<&'static str, &'static FieldSchema> {
        static FIELDS: LazyLock<HashMap<&'static str, &'static FieldSchema>> =
            LazyLock::new(|| {
                let mut map = HashMap::new();
                for f in ALL_FIELDS {
                    map.insert(f.key, f);
                }
                map
            });
        &FIELDS
    }

    /// Look up the schema for a single key, or `None` if unknown.
    pub fn get_field(key: &str) -> Option<&'static FieldSchema> {
        Self::preset_fields().get(key).copied()
    }

    /// Return the `FieldType` for a key, defaulting to `FieldType::String` if unknown.
    pub fn get_key_type(key: &str) -> FieldType {
        Self::preset_fields()
            .get(key)
            .map(|f| f.field_type)
            .unwrap_or(FieldType::String)
    }
}

// ---------------------------------------------------------------------------
// Enum value lists
// ---------------------------------------------------------------------------

static BUTTON_SHAPE_VALUES: &[&str] = &[
    "ShapeSmallCircle",
    "ShapeFullHeightRectangle",
    "ShapeIntegratedRoundedRectangle",
    "ShapeSmallSquare",
    "ShapeFullHeightRoundedRectangle",
];

static BUTTON_ICON_STYLE_VALUES: &[&str] = &[
    "StyleOxygen",
    "StyleKlasse",
    "StyleSuessigKite",
    "StyleRedmond",
    "StyleSystemIconTheme",
    "StyleKlassyKite",
];

static SHADOW_SIZE_VALUES: &[&str] = &[
    "ShadowNone",
    "ShadowSmall",
    "ShadowMedium",
    "ShadowLarge",
    "ShadowVeryLarge",
];

static TITLE_ALIGNMENT_VALUES: &[&str] = &[
    "AlignLeft",
    "AlignCenter",
    "AlignCenterFullWidth",
    "AlignRight",
];

static WINDOW_OUTLINE_STYLE_VALUES: &[&str] = &[
    "WindowOutlineNone",
    "WindowOutlineShadowColor",
    "WindowOutlineCustomColor",
    "WindowOutlineAccentColor",
    "WindowOutlineAccentWithContrast",
    "WindowOutlineCustomWithContrast",
    "WindowOutlineContrastOnly",
];

static BUTTON_BACKGROUND_COLORS_VALUES: &[&str] = &[
    "TitleBarText",
    "TitleBarTextNegativeClose",
    "AccentTrafficLights",
    "AccentWithNegativeClose",
    "Accent",
];

static ICON_SIZE_VALUES: &[&str] = &[
    "IconSmall",
    "IconSmallMedium",
    "IconMedium",
    "IconLargeMedium",
    "IconLarge",
];

static SYSTEM_ICON_SIZE_VALUES: &[&str] = &[
    "SystemIcon16",
    "SystemIcon22",
    "SystemIcon32",
    "SystemIcon48",
];

static KWIN_BORDER_SIZE_VALUES: &[&str] = &[
    "None", "NoSides", "Tiny", "Normal", "Large", "VeryLarge", "Huge", "VeryHuge", "Oversized",
];

static BOLD_BUTTON_ICONS_VALUES: &[&str] = &["BoldIconsFine", "BoldIconsBold"];

static BUTTON_CORNER_RADIUS_VALUES: &[&str] = &["SameAsWindow", "Custom"];

static ON_POOR_ICON_CONTRAST_VALUES: &[&str] = &["TitleBarBackground", "TitleBarText", "No"];

static BUTTON_ICON_COLORS_VALUES: &[&str] =
    &["TitleBarText", "Custom", "WhiteWhenHoverPress"];

static BUTTON_STATE_CHECKED_VALUES: &[&str] = &["Normal", "Hover"];

static VARY_COLOR_VALUES: &[&str] = &["No", "Light", "Dark", "MoreTitleBar", "LessTitleBar"];

// ---------------------------------------------------------------------------
// Master field list (representative subset of all 179 keys)
// ---------------------------------------------------------------------------

static ALL_FIELDS: &[FieldSchema] = &[
    // -- Opacity & title bar appearance --
    FieldSchema {
        key: "ActiveTitleBarOpacity",
        field_type: FieldType::Integer,
        default_value: "100",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Opacity of the active title bar (0 = transparent, 100 = opaque)",
    },
    FieldSchema {
        key: "InactiveTitleBarOpacity",
        field_type: FieldType::Integer,
        default_value: "100",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Opacity of the inactive title bar",
    },
    FieldSchema {
        key: "OpaqueMaximizedTitleBars",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Force opaque title bars on maximized windows",
    },
    FieldSchema {
        key: "OverrideActiveTitleBarOpacity",
        field_type: FieldType::Integer,
        default_value: "0",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Override active title bar opacity (0 = use default)",
    },
    FieldSchema {
        key: "OverrideInactiveTitleBarOpacity",
        field_type: FieldType::Integer,
        default_value: "0",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Override inactive title bar opacity (0 = use default)",
    },
    FieldSchema {
        key: "ApplyOpacityToHeader",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Apply opacity settings to header area",
    },
    FieldSchema {
        key: "BlurTransparentTitleBars",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Blur behind transparent title bars",
    },
    FieldSchema {
        key: "MatchTitleBarToApplicationColor",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Match title bar color to application header bar",
    },
    FieldSchema {
        key: "DrawTitleBarSeparator",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Draw a separator line below the title bar",
    },
    FieldSchema {
        key: "UseTitleBarColorForAllBorders",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Use title bar color for all window borders",
    },
    // -- Title bar spacing --
    FieldSchema {
        key: "TitleBarTopMargin",
        field_type: FieldType::Double,
        default_value: "4.5",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(50.0),
        enum_values: None,
        description: "Top margin of the title bar (pixels)",
    },
    FieldSchema {
        key: "TitleBarBottomMargin",
        field_type: FieldType::Double,
        default_value: "4.5",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(50.0),
        enum_values: None,
        description: "Bottom margin of the title bar (pixels)",
    },
    FieldSchema {
        key: "TitleBarLeftMargin",
        field_type: FieldType::Double,
        default_value: "4.5",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(50.0),
        enum_values: None,
        description: "Left margin of the title bar (pixels)",
    },
    FieldSchema {
        key: "TitleBarRightMargin",
        field_type: FieldType::Double,
        default_value: "4.5",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(50.0),
        enum_values: None,
        description: "Right margin of the title bar (pixels)",
    },
    FieldSchema {
        key: "PercentMaximizedTopBottomMargins",
        field_type: FieldType::Integer,
        default_value: "100",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Percentage of top/bottom margins to keep when maximized",
    },
    FieldSchema {
        key: "LockTitleBarTopBottomMargins",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Lock top and bottom margins to the same value",
    },
    FieldSchema {
        key: "LockTitleBarLeftRightMargins",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Lock left and right margins to the same value",
    },
    // -- Title text --
    FieldSchema {
        key: "TitleAlignment",
        field_type: FieldType::EnumType,
        default_value: "AlignLeft",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(TITLE_ALIGNMENT_VALUES),
        description: "Horizontal alignment of the window title",
    },
    FieldSchema {
        key: "TitleSidePadding",
        field_type: FieldType::Integer,
        default_value: "2",
        min_int: Some(0),
        max_int: Some(50),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Padding on each side of the title text",
    },
    FieldSchema {
        key: "BoldTitle",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Use bold font for the title",
    },
    FieldSchema {
        key: "UnderlineTitle",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Underline the window title text",
    },
    // -- Button shape & style --
    FieldSchema {
        key: "ButtonShape",
        field_type: FieldType::EnumType,
        default_value: "ShapeSmallCircle",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_SHAPE_VALUES),
        description: "Shape of the window decoration buttons",
    },
    FieldSchema {
        key: "ButtonIconStyle",
        field_type: FieldType::EnumType,
        default_value: "StyleKlasse",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_ICON_STYLE_VALUES),
        description: "Icon style for window decoration buttons",
    },
    FieldSchema {
        key: "BoldButtonIcons",
        field_type: FieldType::EnumType,
        default_value: "BoldIconsFine",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BOLD_BUTTON_ICONS_VALUES),
        description: "Thickness of button icons",
    },
    FieldSchema {
        key: "ButtonCornerRadius",
        field_type: FieldType::EnumType,
        default_value: "SameAsWindow",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_CORNER_RADIUS_VALUES),
        description: "Corner radius mode for buttons",
    },
    FieldSchema {
        key: "ButtonCustomCornerRadius",
        field_type: FieldType::Double,
        default_value: "2.0",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(50.0),
        enum_values: None,
        description: "Custom corner radius value when ButtonCornerRadius is Custom",
    },
    FieldSchema {
        key: "ScaleBackgroundPercent",
        field_type: FieldType::Integer,
        default_value: "100",
        min_int: Some(0),
        max_int: Some(200),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Scale factor for button background (percentage)",
    },
    // -- Button background colors --
    FieldSchema {
        key: "ButtonBackgroundColorsActive",
        field_type: FieldType::EnumType,
        default_value: "Accent",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_BACKGROUND_COLORS_VALUES),
        description: "Background color scheme for active window buttons",
    },
    FieldSchema {
        key: "ButtonBackgroundColorsInactive",
        field_type: FieldType::EnumType,
        default_value: "Accent",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_BACKGROUND_COLORS_VALUES),
        description: "Background color scheme for inactive window buttons",
    },
    FieldSchema {
        key: "ButtonBackgroundOpacityActive",
        field_type: FieldType::Integer,
        default_value: "20",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Opacity of button backgrounds on active windows",
    },
    FieldSchema {
        key: "ButtonBackgroundOpacityInactive",
        field_type: FieldType::Integer,
        default_value: "20",
        min_int: Some(0),
        max_int: Some(100),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Opacity of button backgrounds on inactive windows",
    },
    FieldSchema {
        key: "LockButtonColorsActiveInactive",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Lock active/inactive button colors to the same settings",
    },
    // -- Button icon colors --
    FieldSchema {
        key: "ButtonIconColorsActive",
        field_type: FieldType::EnumType,
        default_value: "TitleBarText",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_ICON_COLORS_VALUES),
        description: "Icon color scheme for active window buttons",
    },
    FieldSchema {
        key: "ButtonIconColorsInactive",
        field_type: FieldType::EnumType,
        default_value: "TitleBarText",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_ICON_COLORS_VALUES),
        description: "Icon color scheme for inactive window buttons",
    },
    // -- Shadow --
    FieldSchema {
        key: "ShadowSize",
        field_type: FieldType::EnumType,
        default_value: "ShadowMedium",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(SHADOW_SIZE_VALUES),
        description: "Size of the window shadow",
    },
    FieldSchema {
        key: "ShadowStrength",
        field_type: FieldType::Integer,
        default_value: "160",
        min_int: Some(0),
        max_int: Some(255),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Strength (alpha) of the window shadow",
    },
    FieldSchema {
        key: "ShadowColor",
        field_type: FieldType::Color,
        default_value: "0,0,0",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Color of the window shadow (R,G,B)",
    },
    // -- Window outline --
    FieldSchema {
        key: "WindowOutlineStyleActive",
        field_type: FieldType::EnumType,
        default_value: "WindowOutlineShadowColor",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(WINDOW_OUTLINE_STYLE_VALUES),
        description: "Outline style for active windows",
    },
    FieldSchema {
        key: "WindowOutlineStyleInactive",
        field_type: FieldType::EnumType,
        default_value: "WindowOutlineShadowColor",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(WINDOW_OUTLINE_STYLE_VALUES),
        description: "Outline style for inactive windows",
    },
    FieldSchema {
        key: "WindowOutlineCustomColorActive",
        field_type: FieldType::Color,
        default_value: "0,0,0",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Custom outline color for active windows (R,G,B)",
    },
    FieldSchema {
        key: "WindowOutlineCustomColorInactive",
        field_type: FieldType::Color,
        default_value: "0,0,0",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Custom outline color for inactive windows (R,G,B)",
    },
    FieldSchema {
        key: "WindowOutlineOverlap",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Allow window outline to overlap content",
    },
    FieldSchema {
        key: "WindowOutlineThickness",
        field_type: FieldType::Double,
        default_value: "1.0",
        min_int: None,
        max_int: None,
        min_double: Some(0.0),
        max_double: Some(10.0),
        enum_values: None,
        description: "Thickness of the window outline (pixels)",
    },
    FieldSchema {
        key: "ColorizeWindowOutlineWithButton",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Colorize window outline to match button color",
    },
    // -- Window geometry --
    FieldSchema {
        key: "WindowCornerRadius",
        field_type: FieldType::Integer,
        default_value: "4",
        min_int: Some(0),
        max_int: Some(50),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Corner radius of the window (pixels)",
    },
    FieldSchema {
        key: "KwinBorderSize",
        field_type: FieldType::EnumType,
        default_value: "Normal",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(KWIN_BORDER_SIZE_VALUES),
        description: "KWin border size",
    },
    // -- Animations --
    FieldSchema {
        key: "AnimationsEnabled",
        field_type: FieldType::Boolean,
        default_value: "true",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Enable button animations",
    },
    FieldSchema {
        key: "AnimationsSpeedRelativeSystem",
        field_type: FieldType::Integer,
        default_value: "0",
        min_int: Some(-10),
        max_int: Some(10),
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Animation speed relative to system setting",
    },
    // -- Icon sizes --
    FieldSchema {
        key: "IconSize",
        field_type: FieldType::EnumType,
        default_value: "IconSmallMedium",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(ICON_SIZE_VALUES),
        description: "Button icon size",
    },
    FieldSchema {
        key: "SystemIconSize",
        field_type: FieldType::EnumType,
        default_value: "SystemIcon16",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(SYSTEM_ICON_SIZE_VALUES),
        description: "System icon size used for system icon theme style",
    },
    // -- Style / gradient --
    FieldSchema {
        key: "ButtonGradient",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Draw button background gradient",
    },
    FieldSchema {
        key: "ScrollBarSeparator",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Draw scroll bar separator",
    },
    FieldSchema {
        key: "DrawBackgroundGradient",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Draw background gradient on title bar",
    },
    // -- Poor contrast --
    FieldSchema {
        key: "OnPoorIconContrastActive",
        field_type: FieldType::EnumType,
        default_value: "No",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(ON_POOR_ICON_CONTRAST_VALUES),
        description: "Action to take on poor icon contrast (active)",
    },
    FieldSchema {
        key: "OnPoorIconContrastInactive",
        field_type: FieldType::EnumType,
        default_value: "No",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(ON_POOR_ICON_CONTRAST_VALUES),
        description: "Action to take on poor icon contrast (inactive)",
    },
    // -- VaryColor --
    FieldSchema {
        key: "VaryColorBackgroundActive",
        field_type: FieldType::EnumType,
        default_value: "No",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(VARY_COLOR_VALUES),
        description: "Vary background color per button type (active)",
    },
    FieldSchema {
        key: "VaryColorBackgroundInactive",
        field_type: FieldType::EnumType,
        default_value: "No",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(VARY_COLOR_VALUES),
        description: "Vary background color per button type (inactive)",
    },
    // -- Button checked state --
    FieldSchema {
        key: "ButtonStateCheckedActive",
        field_type: FieldType::EnumType,
        default_value: "Normal",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_STATE_CHECKED_VALUES),
        description: "Appearance of checked (toggled) buttons on active windows",
    },
    FieldSchema {
        key: "ButtonStateCheckedInactive",
        field_type: FieldType::EnumType,
        default_value: "Normal",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: Some(BUTTON_STATE_CHECKED_VALUES),
        description: "Appearance of checked (toggled) buttons on inactive windows",
    },
    // -- Bundled preset flag --
    FieldSchema {
        key: "BundledPreset",
        field_type: FieldType::Boolean,
        default_value: "false",
        min_int: None,
        max_int: None,
        min_double: None,
        max_double: None,
        enum_values: None,
        description: "Whether this is a bundled (read-only) preset",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_lookup() {
        let field = KlassySchema::get_field("WindowCornerRadius").unwrap();
        assert_eq!(field.field_type, FieldType::Integer);
        assert_eq!(field.default_value, "4");
        assert_eq!(field.min_int, Some(0));
        assert_eq!(field.max_int, Some(50));
    }

    #[test]
    fn schema_enum_field() {
        let field = KlassySchema::get_field("ButtonShape").unwrap();
        assert_eq!(field.field_type, FieldType::EnumType);
        assert!(field.enum_values.unwrap().contains(&"ShapeSmallCircle"));
    }

    #[test]
    fn schema_unknown_key() {
        assert!(KlassySchema::get_field("NonExistent").is_none());
        assert_eq!(
            KlassySchema::get_key_type("NonExistent"),
            FieldType::String
        );
    }

    #[test]
    fn schema_has_fields() {
        let fields = KlassySchema::preset_fields();
        assert!(fields.len() > 40);
    }
}
