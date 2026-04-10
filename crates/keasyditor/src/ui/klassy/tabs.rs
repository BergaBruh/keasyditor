use std::collections::{HashMap, HashSet};

use iced::widget::{column, container, row, rule, text, text_input, Space};
use iced::{Element, Fill, Length};

use crate::i18n::t;
use crate::message::Message;
use crate::theme;
use crate::ui::widgets::collapsible::collapsible_section;
use crate::ui::widgets::color_picker::color_picker_field;
use crate::ui::widgets::fields::{enum_dropdown, range_slider_field, toggle_field};

/// Returns the content for a Klassy tab by index.
pub fn klassy_tab_content<'a>(
    tab: usize,
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    match tab {
        0 => buttons_tab(expanded, slider_values, text_input_values),
        1 => titlebar_tab(expanded, slider_values, toggle_values, text_input_values),
        2 => window_tab(expanded, slider_values, toggle_values, text_input_values),
        3 => shadows_tab(expanded, slider_values, text_input_values),
        4 => animations_tab(expanded, slider_values, toggle_values),
        5 => advanced_tab(expanded, slider_values, toggle_values),
        _ => text("Unknown tab").into(),
    }
}

// ── Buttons tab ──────────────────────────────────────────────────────────

fn buttons_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    static SHAPES: &[&str] = &[
        "SmallCircle",
        "SmallSquare",
        "FullHeightRectangle",
        "FullHeightRoundedRectangle",
        "IntegratedRoundedRectangle",
    ];
    static ICON_STYLES: &[&str] = &["Klasse", "Oxygen", "Breeze", "Redmond"];
    static BG_COLORS: &[&str] = &[
        "TitleBarText",
        "Accent",
        "AccentTrafficLights",
        "AccentWithNegativeClose",
        "TitleBarTextNegativeClose",
    ];

    let shape_section = collapsible_section(
        t("klassy.buttons.shape_style"),
        "klassy.buttons.shape",
        expanded.contains("klassy.buttons.shape"),
        theme::AMBER,
        column![
            enum_dropdown(t("klassy.buttons.button_shape"), "klassy.button_shape", text_input_values.get("klassy.button_shape").map(String::as_str), SHAPES),
            Space::new().height(12),
            enum_dropdown(t("klassy.buttons.icon_style"), "klassy.button_icon_style", text_input_values.get("klassy.button_icon_style").map(String::as_str), ICON_STYLES),
        ]
        .spacing(0)
        .into(),
    );

    let spacing_section = collapsible_section(
        t("klassy.buttons.spacing"),
        "klassy.buttons.spacing",
        expanded.contains("klassy.buttons.spacing"),
        theme::AMBER,
        column![
            range_slider_field(
                t("klassy.buttons.spacing_left"),
                "klassy.button_spacing_left",
                slider_values.get("klassy.button_spacing_left").copied().unwrap_or(0.0),
                0.0, 20.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.buttons.spacing_right"),
                "klassy.button_spacing_right",
                slider_values.get("klassy.button_spacing_right").copied().unwrap_or(0.0),
                0.0, 20.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let bg_section = collapsible_section(
        t("klassy.buttons.background"),
        "klassy.buttons.bg",
        expanded.contains("klassy.buttons.bg"),
        theme::AMBER,
        column![
            enum_dropdown(t("klassy.buttons.background_active"), "klassy.bg_colors_active", text_input_values.get("klassy.bg_colors_active").map(String::as_str), BG_COLORS),
            Space::new().height(12),
            enum_dropdown(t("klassy.buttons.background_inactive"), "klassy.bg_colors_inactive", text_input_values.get("klassy.bg_colors_inactive").map(String::as_str), BG_COLORS),
            Space::new().height(12),
            range_slider_field(
                t("klassy.buttons.opacity_active"),
                "klassy.bg_opacity_active",
                slider_values.get("klassy.bg_opacity_active").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.buttons.opacity_inactive"),
                "klassy.bg_opacity_inactive",
                slider_values.get("klassy.bg_opacity_inactive").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
        ]
        .spacing(0)
        .into(),
    );

    // Icon Colors section with color pickers for Close/Minimize/Maximize
    let icon_colors_section = collapsible_section(
        t("klassy.buttons.icon_colors"),
        "klassy.buttons.icon_colors",
        expanded.contains("klassy.buttons.icon_colors"),
        theme::AMBER,
        column![
            color_picker_field(
                t("klassy.buttons.close_color"),
                "klassy.close_icon_color",
                "#FF0000",
                expanded.contains("color.klassy.close_icon_color"),
                slider_values,
                text_input_values,
            ),
            Space::new().height(12),
            color_picker_field(
                t("klassy.buttons.minimize_color"),
                "klassy.minimize_icon_color",
                "#FFFFFF",
                expanded.contains("color.klassy.minimize_icon_color"),
                slider_values,
                text_input_values,
            ),
            Space::new().height(12),
            color_picker_field(
                t("klassy.buttons.maximize_color"),
                "klassy.maximize_icon_color",
                "#FFFFFF",
                expanded.contains("color.klassy.maximize_icon_color"),
                slider_values,
                text_input_values,
            ),
        ]
        .spacing(0)
        .into(),
    );

    column![
        shape_section,
        Space::new().height(12),
        spacing_section,
        Space::new().height(12),
        bg_section,
        Space::new().height(12),
        icon_colors_section,
    ]
    .width(Fill)
    .into()
}

// ── Titlebar tab ─────────────────────────────────────────────────────────

fn titlebar_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    static TITLE_ALIGNMENTS: &[&str] = &["Left", "Center", "Right", "MatchToAppLeft"];

    let opacity_section = collapsible_section(
        t("klassy.titlebar.opacity"),
        "klassy.titlebar.opacity",
        expanded.contains("klassy.titlebar.opacity"),
        theme::AMBER,
        column![
            range_slider_field(
                t("klassy.titlebar.active_opacity"),
                "klassy.titlebar_opacity",
                slider_values.get("klassy.titlebar_opacity").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.titlebar.inactive_opacity"),
                "klassy.titlebar_opacity_inactive",
                slider_values.get("klassy.titlebar_opacity_inactive").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let spacing_section = collapsible_section(
        t("klassy.titlebar.spacing"),
        "klassy.titlebar.spacing",
        expanded.contains("klassy.titlebar.spacing"),
        theme::AMBER,
        column![
            range_slider_field(
                t("klassy.titlebar.side_padding"),
                "klassy.titlebar_side_padding",
                slider_values.get("klassy.titlebar_side_padding").copied().unwrap_or(4.0),
                0.0, 20.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.titlebar.top_margin"),
                "klassy.titlebar_top_margin",
                slider_values.get("klassy.titlebar_top_margin").copied().unwrap_or(0.0),
                0.0, 10.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.titlebar.bottom_margin"),
                "klassy.titlebar_bottom_margin",
                slider_values.get("klassy.titlebar_bottom_margin").copied().unwrap_or(0.0),
                0.0, 10.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.titlebar.left_margin"),
                "klassy.titlebar_left_margin",
                slider_values.get("klassy.titlebar_left_margin").copied().unwrap_or(0.0),
                0.0, 20.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.titlebar.right_margin"),
                "klassy.titlebar_right_margin",
                slider_values.get("klassy.titlebar_right_margin").copied().unwrap_or(0.0),
                0.0, 20.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let style_section = collapsible_section(
        t("klassy.titlebar.title_style"),
        "klassy.titlebar.style",
        expanded.contains("klassy.titlebar.style"),
        theme::AMBER,
        column![
            enum_dropdown(
                t("klassy.titlebar.alignment"),
                "klassy.title_alignment",
                text_input_values.get("klassy.title_alignment").map(String::as_str),
                TITLE_ALIGNMENTS,
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.titlebar.bold"),
                "klassy.bold_title",
                *toggle_values.get("klassy.bold_title").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.titlebar.underline"),
                "klassy.underline_title",
                *toggle_values.get("klassy.underline_title").unwrap_or(&false),
            ),
        ]
        .spacing(0)
        .into(),
    );

    let other_section = collapsible_section(
        t("klassy.titlebar.other"),
        "klassy.titlebar.other",
        expanded.contains("klassy.titlebar.other"),
        theme::AMBER,
        column![
            toggle_field(
                t("klassy.titlebar.match_app"),
                "klassy.match_titlebar_to_app",
                *toggle_values.get("klassy.match_titlebar_to_app").unwrap_or(&false),
            ),
        ]
        .spacing(0)
        .into(),
    );

    column![
        style_section,
        Space::new().height(12),
        opacity_section,
        Space::new().height(12),
        spacing_section,
        Space::new().height(12),
        other_section,
    ]
    .width(Fill)
    .into()
}

// ── Window tab ───────────────────────────────────────────────────────────

fn window_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    static OUTLINE_STYLES: &[&str] = &[
        "None",
        "WindowOutlineAccentColor",
        "WindowOutlineCustomColor",
        "WindowOutlineAccentWithContrast",
    ];
    static BORDER_SIZES: &[&str] = &[
        "None",
        "NoSides",
        "Tiny",
        "Normal",
        "Large",
        "VeryLarge",
        "Oversized",
    ];

    let corner_section = collapsible_section(
        t("klassy.window.corners"),
        "klassy.window.corners",
        expanded.contains("klassy.window.corners"),
        theme::AMBER,
        column![
            range_slider_field(
                t("klassy.window.corner_radius"),
                "klassy.corner_radius",
                slider_values.get("klassy.corner_radius").copied().unwrap_or(10.0),
                0.0, 30.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let outline_section = collapsible_section(
        t("klassy.window.outline"),
        "klassy.window.outline",
        expanded.contains("klassy.window.outline"),
        theme::AMBER,
        column![
            enum_dropdown(t("klassy.window.outline_active"), "klassy.outline_active", text_input_values.get("klassy.outline_active").map(String::as_str), OUTLINE_STYLES),
            Space::new().height(12),
            enum_dropdown(t("klassy.window.outline_inactive"), "klassy.outline_inactive", text_input_values.get("klassy.outline_inactive").map(String::as_str), OUTLINE_STYLES),
            Space::new().height(12),
            toggle_field(
                t("klassy.window.custom_color"),
                "klassy.custom_outline",
                *toggle_values.get("klassy.custom_outline").unwrap_or(&false),
            ),
            Space::new().height(12),
            color_picker_field(
                t("klassy.window.custom_color_active"),
                "klassy.outline_color_active",
                "#000000",
                expanded.contains("color.klassy.outline_color_active"),
                slider_values,
                text_input_values,
            ),
            Space::new().height(12),
            color_picker_field(
                t("klassy.window.custom_color_inactive"),
                "klassy.outline_color_inactive",
                "#000000",
                expanded.contains("color.klassy.outline_color_inactive"),
                slider_values,
                text_input_values,
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.window.custom_opacity_active"),
                "klassy.outline_opacity_active",
                slider_values.get("klassy.outline_opacity_active").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.window.custom_opacity_inactive"),
                "klassy.outline_opacity_inactive",
                slider_values.get("klassy.outline_opacity_inactive").copied().unwrap_or(100.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.window.overlap"),
                "klassy.outline_overlap",
                *toggle_values.get("klassy.outline_overlap").unwrap_or(&false),
            ),
        ]
        .spacing(0)
        .into(),
    );

    let border_section = collapsible_section(
        t("klassy.window.border_size"),
        "klassy.window.border",
        expanded.contains("klassy.window.border"),
        theme::AMBER,
        column![
            enum_dropdown(t("klassy.window.kwin_border"), "klassy.kwin_border_size", text_input_values.get("klassy.kwin_border_size").map(String::as_str), BORDER_SIZES),
        ]
        .spacing(0)
        .into(),
    );

    column![
        corner_section,
        Space::new().height(12),
        outline_section,
        Space::new().height(12),
        border_section,
    ]
    .width(Fill)
    .into()
}

// ── Shadows tab ──────────────────────────────────────────────────────────

fn shadows_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {

    let shadow_section = collapsible_section(
        t("klassy.shadows.style"),
        "klassy.shadows.style",
        expanded.contains("klassy.shadows.style"),
        theme::AMBER,
        column![
            color_picker_field(
                t("klassy.shadows.color"),
                "klassy.shadow_color",
                "#000000",
                expanded.contains("color.klassy.shadow_color"),
                slider_values,
                text_input_values,
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.shadows.strength"),
                "klassy.shadow_strength",
                slider_values.get("klassy.shadow_strength").copied().unwrap_or(128.0),
                0.0, 255.0, 1.0, "",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.shadows.size"),
                "klassy.shadow_size",
                slider_values.get("klassy.shadow_size").copied().unwrap_or(40.0),
                0.0, 100.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    column![shadow_section]
        .width(Fill)
        .into()
}

// ── Animations tab ───────────────────────────────────────────────────────

fn animations_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
) -> Element<'a, Message> {
    let anim_section = collapsible_section(
        t("klassy.animations.settings"),
        "klassy.animations.settings",
        expanded.contains("klassy.animations.settings"),
        theme::AMBER,
        column![
            toggle_field(
                t("klassy.animations.enabled"),
                "klassy.animations_enabled",
                *toggle_values.get("klassy.animations_enabled").unwrap_or(&true),
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.animations.duration"),
                "klassy.anim_duration",
                slider_values.get("klassy.anim_duration").copied().unwrap_or(150.0),
                0.0, 500.0, 10.0, "ms",
            ),
            Space::new().height(12),
            range_slider_field(
                t("klassy.animations.hover_opacity"),
                "klassy.hover_opacity",
                slider_values.get("klassy.hover_opacity").copied().unwrap_or(20.0),
                0.0, 100.0, 1.0, "%",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let hover_section = collapsible_section(
        t("klassy.animations.hover_behaviour"),
        "klassy.animations.hover",
        expanded.contains("klassy.animations.hover"),
        theme::AMBER,
        column![
            toggle_field(
                t("klassy.animations.unison"),
                "klassy.unison_hovering",
                *toggle_values.get("klassy.unison_hovering").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.animations.accent_active"),
                "klassy.hover_accent_active",
                *toggle_values.get("klassy.hover_accent_active").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.animations.accent_inactive"),
                "klassy.hover_accent_inactive",
                *toggle_values.get("klassy.hover_accent_inactive").unwrap_or(&false),
            ),
        ]
        .spacing(0)
        .into(),
    );

    column![
        anim_section,
        Space::new().height(12),
        hover_section,
    ]
    .width(Fill)
    .into()
}

// ── Advanced tab ─────────────────────────────────────────────────────────

/// Known Klassy INI sections and their common keys for the raw key=value editor.
const KLASSY_SECTIONS: &[(&str, &[&str])] = &[
    ("ButtonColors", &[
        "ButtonBackgroundColorsActive",
        "ButtonBackgroundColorsInactive",
        "ButtonBackgroundOpacityActive",
        "ButtonBackgroundOpacityInactive",
        "CloseButtonCustomIconColor",
        "MinimizeButtonCustomIconColor",
        "MaximizeButtonCustomIconColor",
        "UseHoverAccentActive",
        "UseHoverAccentInactive",
    ]),
    ("Global", &[
        "ButtonsOnLeft",
    ]),
    ("ShadowStyle", &[
        "ShadowColor",
        "ShadowSize",
        "ShadowStrength",
    ]),
    ("Style", &[
        "ButtonStyle",
    ]),
    ("TitleBarOpacity", &[
        "ActiveTitleBarOpacity",
        "InactiveTitleBarOpacity",
        "OpaqueMaximizedTitleBars",
    ]),
    ("TitleBarSpacing", &[
        "TitleBarTopMargin",
        "TitleBarBottomMargin",
        "TitleBarLeftMargin",
        "TitleBarRightMargin",
        "TitleBarSidePadding",
    ]),
    ("Windeco", &[
        "AnimationsEnabled",
        "AnimationsSpeedRelativeSystem",
        "BlurTransparentTitleBars",
        "BoldTitle",
        "ButtonIconStyle",
        "ButtonShape",
        "ButtonSpacingLeft",
        "ButtonSpacingRight",
        "KwinBorderSize",
        "MatchTitleBarToApplicationColor",
        "TitleAlignment",
        "UnderlineTitle",
        "UnisonHovering",
        "WindowCornerRadius",
    ]),
    ("WindowOutlineStyle", &[
        "WindowOutlineCustomColorActive",
        "WindowOutlineCustomColorInactive",
        "WindowOutlineCustomColorOpacityActive",
        "WindowOutlineCustomColorOpacityInactive",
        "WindowOutlineOverlap",
        "WindowOutlineStyleActive",
        "WindowOutlineStyleInactive",
    ]),
];

fn advanced_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
) -> Element<'a, Message> {
    // Search field at the top
    // We store the search query in slider_values using a sentinel key.
    // Since text_input_values is not available here, we use a static empty string
    // for the search placeholder and rely on TextInputChanged messages.
    // The search key in text_input_values will be "klassy.advanced.search".
    // However, since we can't read it back, we show all keys.

    let search_key = "klassy.advanced.search".to_string();
    let filter_placeholder = t("klassy.advanced.filter_keys");
    let search_input = container(
        text_input(&filter_placeholder, "")
            .on_input(move |v| Message::TextInputChanged {
                key: search_key.clone(),
                value: v,
            })
            .size(13)
            .width(Fill),
    )
    .width(Fill)
    .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 });

    // General toggles section (existing)
    let general_section = collapsible_section(
        t("klassy.advanced.general"),
        "klassy.advanced.general",
        expanded.contains("klassy.advanced.general"),
        theme::AMBER,
        column![
            toggle_field(
                t("klassy.advanced.draw_border_maximized"),
                "klassy.border_maximized",
                *toggle_values.get("klassy.border_maximized").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.advanced.draw_separator"),
                "klassy.titlebar_separator",
                *toggle_values.get("klassy.titlebar_separator").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                t("klassy.advanced.draw_highlight"),
                "klassy.active_highlight",
                *toggle_values.get("klassy.active_highlight").unwrap_or(&true),
            ),
        ]
        .spacing(0)
        .into(),
    );

    // Raw key=value editor: scrollable list of all config sections
    let mut raw_sections = column![].spacing(12).width(Fill);

    for (section_name, keys) in KLASSY_SECTIONS {
        let section_id = format!("klassy.advanced.raw.{}", section_name);
        let is_expanded = expanded.contains(&section_id);

        let mut key_rows = column![].spacing(4);
        for key_name in *keys {
            let full_key = format!("klassy.advanced.{}.{}", section_name, key_name);
            // Read the current value from slider_values (as text) or show empty
            let current_val = slider_values
                .get(&full_key)
                .map(|v| {
                    if v.fract().abs() < 0.01 {
                        format!("{}", *v as i32)
                    } else {
                        format!("{:.1}", v)
                    }
                })
                .unwrap_or_default();

            let key_owned = full_key.clone();
            let kv_row = row![
                text(*key_name).size(12).color(theme::DIM).width(Length::FillPortion(2)),
                Space::new().width(8),
                container(
                    text_input("", &current_val)
                        .on_input(move |v| Message::TextInputChanged {
                            key: key_owned.clone(),
                            value: v,
                        })
                        .size(12)
                        .width(Fill)
                )
                .width(Length::FillPortion(2)),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center);

            key_rows = key_rows.push(kv_row);
        }

        let section_content: Element<'_, Message> = key_rows.into();

        let raw_section = collapsible_section(
            *section_name,
            &section_id,
            is_expanded,
            theme::AMBER,
            section_content,
        );

        raw_sections = raw_sections.push(raw_section);
    }

    let raw_editor_section = collapsible_section(
        t("klassy.advanced.raw_config"),
        "klassy.advanced.raw",
        expanded.contains("klassy.advanced.raw"),
        theme::AMBER,
        column![
            search_input,
            rule::horizontal(1),
            Space::new().height(8),
            raw_sections,
        ]
        .spacing(0)
        .into(),
    );

    column![
        general_section,
        Space::new().height(12),
        raw_editor_section,
    ]
    .width(Fill)
    .into()
}
