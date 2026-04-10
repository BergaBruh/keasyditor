/// Enum types for Klassy window decoration configuration values.
///
/// Each enum has `from_value` and `value` methods matching the exact strings
/// used in Klassy INI config files.

macro_rules! klassy_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $variant:ident => $value:expr ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $variant, )+
        }

        impl $name {
            /// Parse from the INI string value.
            pub fn from_value(s: &str) -> Option<Self> {
                match s {
                    $( $value => Some(Self::$variant), )+
                    _ => None,
                }
            }

            /// Return the INI string value.
            pub fn value(&self) -> &'static str {
                match self {
                    $( Self::$variant => $value, )+
                }
            }

            /// Return all variants.
            pub fn all() -> &'static [Self] {
                &[ $( Self::$variant, )+ ]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.value())
            }
        }
    };
}

klassy_enum! {
    /// Shape of window decoration buttons.
    ButtonShape {
        SmallCircle => "ShapeSmallCircle",
        FullHeightRectangle => "ShapeFullHeightRectangle",
        IntegratedRoundedRectangle => "ShapeIntegratedRoundedRectangle",
        SmallSquare => "ShapeSmallSquare",
        FullHeightRoundedRectangle => "ShapeFullHeightRoundedRectangle",
    }
}

klassy_enum! {
    /// Icon style for window decoration buttons.
    ButtonIconStyle {
        Oxygen => "StyleOxygen",
        Klasse => "StyleKlasse",
        SuessigKite => "StyleSuessigKite",
        Redmond => "StyleRedmond",
        SystemIconTheme => "StyleSystemIconTheme",
        KlassyKite => "StyleKlassyKite",
    }
}

klassy_enum! {
    /// Size of the window shadow.
    ShadowSize {
        None => "ShadowNone",
        Small => "ShadowSmall",
        Medium => "ShadowMedium",
        Large => "ShadowLarge",
        VeryLarge => "ShadowVeryLarge",
    }
}

klassy_enum! {
    /// Horizontal alignment of the window title.
    TitleAlignment {
        Left => "AlignLeft",
        Center => "AlignCenter",
        CenterFullWidth => "AlignCenterFullWidth",
        Right => "AlignRight",
    }
}

klassy_enum! {
    /// Window outline style.
    WindowOutlineStyle {
        None => "WindowOutlineNone",
        ShadowColor => "WindowOutlineShadowColor",
        CustomColor => "WindowOutlineCustomColor",
        AccentColor => "WindowOutlineAccentColor",
        AccentWithContrast => "WindowOutlineAccentWithContrast",
        CustomWithContrast => "WindowOutlineCustomWithContrast",
        ContrastOnly => "WindowOutlineContrastOnly",
    }
}

klassy_enum! {
    /// Background color scheme for window decoration buttons.
    ButtonBackgroundColors {
        TitleBarText => "TitleBarText",
        TitleBarTextNegativeClose => "TitleBarTextNegativeClose",
        AccentTrafficLights => "AccentTrafficLights",
        AccentWithNegativeClose => "AccentWithNegativeClose",
        Accent => "Accent",
    }
}

klassy_enum! {
    /// Button icon size.
    IconSize {
        Small => "IconSmall",
        SmallMedium => "IconSmallMedium",
        Medium => "IconMedium",
        LargeMedium => "IconLargeMedium",
        Large => "IconLarge",
    }
}

klassy_enum! {
    /// System icon size for system icon theme style.
    SystemIconSize {
        Icon16 => "SystemIcon16",
        Icon22 => "SystemIcon22",
        Icon32 => "SystemIcon32",
        Icon48 => "SystemIcon48",
    }
}

klassy_enum! {
    /// KWin border size.
    KwinBorderSize {
        None => "None",
        NoSides => "NoSides",
        Tiny => "Tiny",
        Normal => "Normal",
        Large => "Large",
        VeryLarge => "VeryLarge",
        Huge => "Huge",
        VeryHuge => "VeryHuge",
        Oversized => "Oversized",
    }
}

klassy_enum! {
    /// Thickness of button icons.
    BoldButtonIcons {
        Fine => "BoldIconsFine",
        Bold => "BoldIconsBold",
    }
}

klassy_enum! {
    /// Corner radius mode for buttons.
    ButtonCornerRadius {
        SameAsWindow => "SameAsWindow",
        Custom => "Custom",
    }
}

klassy_enum! {
    /// Action to take on poor icon contrast.
    OnPoorIconContrast {
        TitleBarBackground => "TitleBarBackground",
        TitleBarText => "TitleBarText",
        No => "No",
    }
}

klassy_enum! {
    /// Icon color scheme for window buttons.
    ButtonIconColors {
        TitleBarText => "TitleBarText",
        Custom => "Custom",
        WhiteWhenHoverPress => "WhiteWhenHoverPress",
    }
}

klassy_enum! {
    /// Appearance of checked (toggled) buttons.
    ButtonStateChecked {
        Normal => "Normal",
        Hover => "Hover",
    }
}

klassy_enum! {
    /// Vary color per button type.
    VaryColor {
        No => "No",
        Light => "Light",
        Dark => "Dark",
        MoreTitleBar => "MoreTitleBar",
        LessTitleBar => "LessTitleBar",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_shape_round_trip() {
        for variant in ButtonShape::all() {
            let s = variant.value();
            let parsed = ButtonShape::from_value(s).unwrap();
            assert_eq!(*variant, parsed);
        }
    }

    #[test]
    fn button_icon_style_round_trip() {
        for variant in ButtonIconStyle::all() {
            let s = variant.value();
            let parsed = ButtonIconStyle::from_value(s).unwrap();
            assert_eq!(*variant, parsed);
        }
    }

    #[test]
    fn shadow_size_round_trip() {
        for variant in ShadowSize::all() {
            assert_eq!(ShadowSize::from_value(variant.value()), Some(*variant));
        }
    }

    #[test]
    fn window_outline_style_round_trip() {
        for variant in WindowOutlineStyle::all() {
            assert_eq!(
                WindowOutlineStyle::from_value(variant.value()),
                Some(*variant)
            );
        }
    }

    #[test]
    fn kwin_border_size_round_trip() {
        for variant in KwinBorderSize::all() {
            assert_eq!(
                KwinBorderSize::from_value(variant.value()),
                Some(*variant)
            );
        }
    }

    #[test]
    fn unknown_value_returns_none() {
        assert_eq!(ButtonShape::from_value("NonExistent"), None);
        assert_eq!(ShadowSize::from_value(""), None);
        assert_eq!(TitleAlignment::from_value("invalid"), None);
    }

    #[test]
    fn display_matches_value() {
        assert_eq!(
            format!("{}", ButtonShape::SmallCircle),
            "ShapeSmallCircle"
        );
        assert_eq!(
            format!("{}", ShadowSize::Medium),
            "ShadowMedium"
        );
    }

    #[test]
    fn all_enums_have_variants() {
        assert_eq!(ButtonShape::all().len(), 5);
        assert_eq!(ButtonIconStyle::all().len(), 6);
        assert_eq!(ShadowSize::all().len(), 5);
        assert_eq!(TitleAlignment::all().len(), 4);
        assert_eq!(WindowOutlineStyle::all().len(), 7);
        assert_eq!(ButtonBackgroundColors::all().len(), 5);
        assert_eq!(IconSize::all().len(), 5);
        assert_eq!(SystemIconSize::all().len(), 4);
        assert_eq!(KwinBorderSize::all().len(), 9);
        assert_eq!(BoldButtonIcons::all().len(), 2);
        assert_eq!(ButtonCornerRadius::all().len(), 2);
        assert_eq!(OnPoorIconContrast::all().len(), 3);
        assert_eq!(ButtonIconColors::all().len(), 3);
        assert_eq!(ButtonStateChecked::all().len(), 2);
        assert_eq!(VaryColor::all().len(), 5);
    }
}
