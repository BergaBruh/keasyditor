use std::collections::{HashMap, HashSet};

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::Message;
use crate::theme;
use crate::ui::widgets::collapsible::collapsible_section;
use crate::ui::widgets::color_picker::color_picker_field;
use crate::ui::widgets::fields::{enum_dropdown, range_slider_field, toggle_field};

/// Returns the content for a Kvantum tab by index.
pub fn kvantum_tab_content<'a>(
    tab: usize,
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    match tab {
        0 => colors_tab(expanded, slider_values, text_input_values),
        1 => general_tab(expanded, slider_values, toggle_values, text_input_values),
        2 => hacks_tab(expanded, toggle_values, slider_values, text_input_values),
        3 => widgets_tab(expanded, slider_values, text_input_values),
        _ => text("Unknown tab").into(),
    }
}

/// Build a collapsible color section from a list of (key_suffix, label) pairs.
/// Uses the full `color_picker_field` with HSL wheel and RGB sliders.
/// Labels are owned Strings to support i18n translation.
fn build_color_section<'a>(
    title: String,
    section_id: &str,
    expanded: &HashSet<String>,
    color_defs: &[(&str, String)],
    slider_values: &HashMap<String, f32>,
    text_input_values: &HashMap<String, String>,
) -> Element<'a, Message> {
    let default_hex = "#808080";
    let mut col = column![].spacing(10);

    for (key_suffix, label) in color_defs {
        let key_prefix = format!("kvantum.color.{}", key_suffix);
        let hex_key = format!("color.{}.hex", key_prefix);
        let current_hex = text_input_values
            .get(&hex_key)
            .map(String::as_str)
            .or_else(|| text_input_values.get(&key_prefix).map(String::as_str))
            .unwrap_or(default_hex);
        let toggle_key = format!("color.{}", key_prefix);
        col = col.push(color_picker_field(
            label.clone(),
            key_prefix.as_str(),
            current_hex,
            expanded.contains(&toggle_key),
            slider_values,
            text_input_values,
        ));
    }

    collapsible_section(
        title,
        section_id,
        expanded.contains(section_id),
        theme::GREEN,
        col.into(),
    )
}

// ── Colors tab ───────────────────────────────────────────────────────────

fn colors_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let bg_colors: Vec<(&str, String)> = vec![
        ("window.color", t("kvantum.colors.window")),
        ("base.color", t("kvantum.colors.base")),
        ("alt.base.color", t("kvantum.colors.alt_base")),
        ("button.color", t("kvantum.colors.button")),
    ];

    let shading_colors: Vec<(&str, String)> = vec![
        ("light.color", t("kvantum.colors.light")),
        ("mid.light.color", t("kvantum.colors.mid_light")),
        ("dark.color", t("kvantum.colors.dark")),
        ("mid.color", t("kvantum.colors.mid")),
        ("shadow.color", t("kvantum.colors.shadow")),
    ];

    let highlight_colors: Vec<(&str, String)> = vec![
        ("highlight.color", t("kvantum.colors.highlight")),
    ];

    let text_colors: Vec<(&str, String)> = vec![
        ("text.color", t("kvantum.colors.text")),
        ("window.text.color", t("kvantum.colors.window_text")),
        ("button.text.color", t("kvantum.colors.button_text")),
        ("tooltip.text.color", t("kvantum.colors.tooltip_text")),
        ("highlight.text.color", t("kvantum.colors.highlight_text")),
    ];

    let link_colors: Vec<(&str, String)> = vec![
        ("link.color", t("kvantum.colors.link")),
        ("link.visited.color", t("kvantum.colors.visited_link")),
    ];

    let other_colors: Vec<(&str, String)> = vec![
        ("tooltip.base.color", t("kvantum.colors.tooltip_base")),
        ("placeholder.text.color", t("kvantum.colors.placeholder_text")),
    ];

    column![
        build_color_section(t("kvantum.colors.group.backgrounds"), "kvantum.colors.backgrounds", expanded, &bg_colors, slider_values, text_input_values),
        Space::new().height(12),
        build_color_section(t("kvantum.colors.group.shading"), "kvantum.colors.shading", expanded, &shading_colors, slider_values, text_input_values),
        Space::new().height(12),
        build_color_section(t("kvantum.colors.group.highlights"), "kvantum.colors.highlights", expanded, &highlight_colors, slider_values, text_input_values),
        Space::new().height(12),
        build_color_section(t("kvantum.colors.group.text"), "kvantum.colors.text", expanded, &text_colors, slider_values, text_input_values),
        Space::new().height(12),
        build_color_section(t("kvantum.colors.group.links"), "kvantum.colors.links", expanded, &link_colors, slider_values, text_input_values),
        Space::new().height(12),
        build_color_section(t("kvantum.colors.group.other"), "kvantum.colors.other", expanded, &other_colors, slider_values, text_input_values),
    ]
    .width(Fill)
    .into()
}

// ── General tab ──────────────────────────────────────────────────────────

fn general_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    toggle_values: &HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    static X11DRAG_OPTIONS: &[&str] = &["all", "menubar", "none"];
    static CLICK_OPTIONS: &[&str] = &["0", "1"];

    let lbl_behavior = t("kvantum.general.behavior");
    let lbl_visual = t("kvantum.general.visual");
    let lbl_sizing = t("kvantum.general.sizing");
    let lbl_layout = t("kvantum.general.layout");
    let lbl_effects = t("kvantum.general.effects");

    let lbl_x11_drag = t("kvantum.general.x11_drag");
    let lbl_click_behavior = t("kvantum.general.click_behavior");
    let lbl_left_mouse = t("kvantum.general.left_mouse");
    let lbl_scrollbar_in_view = t("kvantum.general.scrollbar_in_view");
    let lbl_scroll_arrows = t("kvantum.general.scroll_arrows");
    let lbl_scrollable_menu = t("kvantum.general.scrollable_menu");
    let lbl_alt_mnemonic = t("kvantum.general.alt_mnemonic");
    let lbl_drag_from_buttons = t("kvantum.general.drag_from_buttons");
    let lbl_double_click = t("kvantum.general.double_click");

    let lbl_composite = t("kvantum.general.composite");
    let lbl_translucent = t("kvantum.general.translucent");
    let lbl_blurring = t("kvantum.general.blurring");
    let lbl_popup_blurring = t("kvantum.general.popup_blurring");
    let lbl_animate = t("kvantum.general.animate");
    let lbl_fill_rubberband = t("kvantum.general.fill_rubberband");
    let lbl_no_window_pattern = t("kvantum.general.no_window_pattern");
    let lbl_shadowless_popup = t("kvantum.general.shadowless_popup");
    let lbl_window_opacity = t("kvantum.general.window_opacity");
    let lbl_menu_opacity = t("kvantum.general.menu_opacity");
    let lbl_contrast = t("kvantum.general.contrast");
    let lbl_intensity = t("kvantum.general.intensity");
    let lbl_saturation = t("kvantum.general.saturation");

    let lbl_small_icon = t("kvantum.general.small_icon");
    let lbl_large_icon = t("kvantum.general.large_icon");
    let lbl_slider_width = t("kvantum.general.slider_width");
    let lbl_slider_handle_width = t("kvantum.general.slider_handle_width");
    let lbl_slider_handle_length = t("kvantum.general.slider_handle_length");

    let lbl_layout_spacing = t("kvantum.general.layout_spacing");
    let lbl_layout_margin = t("kvantum.general.layout_margin");
    let lbl_submenu_overlap = t("kvantum.general.submenu_overlap");

    let lbl_menu_shadow = t("kvantum.general.menu_shadow");
    let lbl_tooltip_shadow = t("kvantum.general.tooltip_shadow");
    let lbl_splitter_width = t("kvantum.general.splitter_width");
    let lbl_scroll_width = t("kvantum.general.scroll_width");
    let lbl_arrow_size = t("kvantum.general.arrow_size");

    let behavior_section = collapsible_section(
        lbl_behavior,
        "kvantum.general.behavior",
        expanded.contains("kvantum.general.behavior"),
        theme::GREEN,
        column![
            enum_dropdown(lbl_x11_drag, "kvantum.general.x11drag",
                text_input_values.get("kvantum.general.x11drag").map(String::as_str), X11DRAG_OPTIONS),
            Space::new().height(12),
            enum_dropdown(lbl_click_behavior, "kvantum.general.click_behavior",
                text_input_values.get("kvantum.general.click_behavior").map(String::as_str), CLICK_OPTIONS),
            Space::new().height(12),
            toggle_field(
                lbl_left_mouse,
                "kvantum.general.left_handed_mouse",
                *toggle_values.get("kvantum.general.left_handed_mouse").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_scrollbar_in_view,
                "kvantum.general.scrollbar_in_view",
                *toggle_values.get("kvantum.general.scrollbar_in_view").unwrap_or(&true),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_scroll_arrows,
                "kvantum.general.scroll_arrows",
                *toggle_values.get("kvantum.general.scroll_arrows").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_scrollable_menu,
                "kvantum.general.scrollable_menu",
                *toggle_values.get("kvantum.general.scrollable_menu").unwrap_or(&true),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_alt_mnemonic,
                "kvantum.general.alt_mnemonic",
                *toggle_values.get("kvantum.general.alt_mnemonic").unwrap_or(&true),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_drag_from_buttons,
                "kvantum.general.drag_from_buttons",
                *toggle_values.get("kvantum.general.drag_from_buttons").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_double_click,
                "kvantum.general.double_click",
                *toggle_values.get("kvantum.general.double_click").unwrap_or(&false),
            ),
        ]
        .spacing(0)
        .into(),
    );

    let visual_section = collapsible_section(
        lbl_visual,
        "kvantum.general.visual",
        expanded.contains("kvantum.general.visual"),
        theme::GREEN,
        column![
            toggle_field(
                lbl_composite,
                "kvantum.general.composite",
                *toggle_values.get("kvantum.general.composite").unwrap_or(&true),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_translucent,
                "kvantum.general.translucent_windows",
                *toggle_values.get("kvantum.general.translucent_windows").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_blurring,
                "kvantum.general.blurring",
                *toggle_values.get("kvantum.general.blurring").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_popup_blurring,
                "kvantum.general.popup_blurring",
                *toggle_values.get("kvantum.general.popup_blurring").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_animate,
                "kvantum.general.animate_states",
                *toggle_values.get("kvantum.general.animate_states").unwrap_or(&true),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_fill_rubberband,
                "kvantum.general.fill_rubberband",
                *toggle_values.get("kvantum.general.fill_rubberband").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_no_window_pattern,
                "kvantum.general.no_window_pattern",
                *toggle_values.get("kvantum.general.no_window_pattern").unwrap_or(&false),
            ),
            Space::new().height(12),
            toggle_field(
                lbl_shadowless_popup,
                "kvantum.general.shadowless_popup",
                *toggle_values.get("kvantum.general.shadowless_popup").unwrap_or(&false),
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_window_opacity,
                "kvantum.window_opacity_reduction",
                slider_values.get("kvantum.window_opacity_reduction").copied().unwrap_or(0.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_menu_opacity,
                "kvantum.reduce_opacity",
                slider_values.get("kvantum.reduce_opacity").copied().unwrap_or(0.0),
                0.0, 100.0, 1.0, "%",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_contrast,
                "kvantum.general.contrast",
                slider_values.get("kvantum.general.contrast").copied().unwrap_or(10.0),
                0.0, 20.0, 1.0, "",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_intensity,
                "kvantum.general.intensity",
                slider_values.get("kvantum.general.intensity").copied().unwrap_or(10.0),
                0.0, 20.0, 1.0, "",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_saturation,
                "kvantum.general.saturation",
                slider_values.get("kvantum.general.saturation").copied().unwrap_or(10.0),
                0.0, 20.0, 1.0, "",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let sizing_section = collapsible_section(
        lbl_sizing,
        "kvantum.general.sizing",
        expanded.contains("kvantum.general.sizing"),
        theme::GREEN,
        column![
            range_slider_field(
                lbl_small_icon,
                "kvantum.general.small_icon_size",
                slider_values.get("kvantum.general.small_icon_size").copied().unwrap_or(16.0),
                16.0, 32.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_large_icon,
                "kvantum.general.large_icon_size",
                slider_values.get("kvantum.general.large_icon_size").copied().unwrap_or(32.0),
                22.0, 64.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_slider_width,
                "kvantum.general.slider_width",
                slider_values.get("kvantum.general.slider_width").copied().unwrap_or(8.0),
                2.0, 24.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_slider_handle_width,
                "kvantum.general.slider_handle_width",
                slider_values.get("kvantum.general.slider_handle_width").copied().unwrap_or(20.0),
                16.0, 40.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_slider_handle_length,
                "kvantum.general.slider_handle_length",
                slider_values.get("kvantum.general.slider_handle_length").copied().unwrap_or(20.0),
                16.0, 40.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let layout_section = collapsible_section(
        lbl_layout,
        "kvantum.general.layout",
        expanded.contains("kvantum.general.layout"),
        theme::GREEN,
        column![
            range_slider_field(
                lbl_layout_spacing,
                "kvantum.general.layout_spacing",
                slider_values.get("kvantum.general.layout_spacing").copied().unwrap_or(6.0),
                2.0, 12.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_layout_margin,
                "kvantum.general.layout_margin",
                slider_values.get("kvantum.general.layout_margin").copied().unwrap_or(4.0),
                2.0, 16.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_submenu_overlap,
                "kvantum.general.submenu_overlap",
                slider_values.get("kvantum.general.submenu_overlap").copied().unwrap_or(0.0),
                -10.0, 10.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    let effects_section = collapsible_section(
        lbl_effects,
        "kvantum.general.effects",
        expanded.contains("kvantum.general.effects"),
        theme::GREEN,
        column![
            range_slider_field(
                lbl_menu_shadow,
                "kvantum.general.menu_shadow_depth",
                slider_values.get("kvantum.general.menu_shadow_depth").copied().unwrap_or(6.0),
                0.0, 30.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_tooltip_shadow,
                "kvantum.general.tooltip_shadow_depth",
                slider_values.get("kvantum.general.tooltip_shadow_depth").copied().unwrap_or(6.0),
                0.0, 20.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_splitter_width,
                "kvantum.general.splitter_width",
                slider_values.get("kvantum.general.splitter_width").copied().unwrap_or(7.0),
                1.0, 12.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_scroll_width,
                "kvantum.general.scroll_width",
                slider_values.get("kvantum.general.scroll_width").copied().unwrap_or(12.0),
                4.0, 30.0, 1.0, "px",
            ),
            Space::new().height(12),
            range_slider_field(
                lbl_arrow_size,
                "kvantum.general.arrow_size",
                slider_values.get("kvantum.general.arrow_size").copied().unwrap_or(9.0),
                6.0, 24.0, 1.0, "px",
            ),
        ]
        .spacing(0)
        .into(),
    );

    column![
        behavior_section,
        Space::new().height(12),
        visual_section,
        Space::new().height(12),
        sizing_section,
        Space::new().height(12),
        layout_section,
        Space::new().height(12),
        effects_section,
    ]
    .width(Fill)
    .into()
}

// ── Hacks tab ────────────────────────────────────────────────────────────

/// All hack toggle keys and their i18n translation keys.
const HACK_TOGGLES: &[(&str, &str)] = &[
    ("transparent_dolphin_view", "kvantum.hacks.transparent_dolphin"),
    ("transparent_ktitle_label", "kvantum.hacks.transparent_ktitle"),
    ("transparent_menu_title", "kvantum.hacks.transparent_menu_title"),
    ("blur_translucent", "kvantum.hacks.blur_translucent"),
    ("respect_DE", "kvantum.hacks.respect_desktop"),
    ("force_size_grip", "kvantum.hacks.force_size_grip"),
    ("middle_click_scroll", "kvantum.hacks.middle_click_scroll"),
    ("normal_default", "kvantum.hacks.normal_default_button"),
    ("iconless_pushbutton", "kvantum.hacks.iconless_push_buttons"),
    ("iconless_menu", "kvantum.hacks.iconless_menus"),
    ("single_top_toolbar", "kvantum.hacks.single_toolbar"),
    ("no_inactive_tab_separator", "kvantum.hacks.no_inactive_tab"),
    ("transparent_arrow_button", "kvantum.hacks.transparent_arrow"),
    ("tint_current_tab", "kvantum.hacks.tint_current_tab"),
    ("center_toolbar_handle", "kvantum.hacks.center_toolbar"),
    ("joined_inactive_tabs", "kvantum.hacks.joined_inactive_tabs"),
];

fn hacks_tab<'a>(
    expanded: &HashSet<String>,
    toggle_values: &HashMap<String, bool>,
    slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let search_key = "kvantum.hacks.search";
    let empty_search = String::new();
    let search_value = text_input_values
        .get(search_key)
        .unwrap_or(&empty_search)
        .clone();
    let search_lower = search_value.to_lowercase();

    let search_placeholder = t("kvantum.hacks.search_placeholder");

    // Search field
    let search_field = container(
        text_input(&search_placeholder, &search_value)
            .on_input(|v| Message::TextInputChanged {
                key: search_key.to_string(),
                value: v,
            })
            .size(13)
            .width(Fill),
    )
    .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 12.0, left: 0.0 });

    // Collect filtered toggle hacks (full_key must outlive the column elements)
    let toggle_items: Vec<(String, String, bool)> = HACK_TOGGLES
        .iter()
        .filter_map(|&(key, i18n_key)| {
            let label = t(i18n_key);
            let label_lower = label.to_lowercase();
            if !search_lower.is_empty()
                && !key.to_lowercase().contains(&search_lower)
                && !label_lower.contains(&search_lower)
            {
                return None;
            }
            let full_key = format!("kvantum.hacks.{}", key);
            let value = *toggle_values.get(full_key.as_str()).unwrap_or(&false);
            Some((full_key, label, value))
        })
        .collect();

    let mut toggle_col = column![].spacing(10);
    for (full_key, label, value) in &toggle_items {
        toggle_col = toggle_col.push(toggle_field(label.clone(), full_key.as_str(), *value));
    }

    let compat_section = collapsible_section(
        t("kvantum.hacks.compat_title"),
        "kvantum.hacks.compat",
        expanded.contains("kvantum.hacks.compat"),
        theme::GREEN,
        toggle_col.into(),
    );

    // Slider hacks
    let mut slider_col = column![].spacing(12);
    let mut has_slider_items = false;

    let lbl_kcapacitybar_width = t("kvantum.hacks.kcapacitybar_width");
    let lbl_lxqt_icon_size = t("kvantum.hacks.lxqt_icon_size");
    let lbl_disabled_icon_opacity = t("kvantum.hacks.disabled_icon_opacity");

    if search_lower.is_empty()
        || "kcapacitybar_width".contains(&search_lower)
        || lbl_kcapacitybar_width.to_lowercase().contains(&search_lower)
    {
        slider_col = slider_col.push(range_slider_field(
            lbl_kcapacitybar_width.clone(),
            "kvantum.hacks.kcapacitybar_width",
            slider_values
                .get("kvantum.hacks.kcapacitybar_width")
                .copied()
                .unwrap_or(0.0),
            0.0, 100.0, 1.0, "px",
        ));
        has_slider_items = true;
    }
    if search_lower.is_empty()
        || "lxqtmainmenu_iconsize".contains(&search_lower)
        || lbl_lxqt_icon_size.to_lowercase().contains(&search_lower)
    {
        slider_col = slider_col.push(range_slider_field(
            lbl_lxqt_icon_size.clone(),
            "kvantum.hacks.lxqtmainmenu_iconsize",
            slider_values
                .get("kvantum.hacks.lxqtmainmenu_iconsize")
                .copied()
                .unwrap_or(22.0),
            8.0, 64.0, 1.0, "px",
        ));
        has_slider_items = true;
    }
    if search_lower.is_empty()
        || "disabled_icon_opacity".contains(&search_lower)
        || lbl_disabled_icon_opacity.to_lowercase().contains(&search_lower)
    {
        slider_col = slider_col.push(range_slider_field(
            lbl_disabled_icon_opacity.clone(),
            "kvantum.hacks.disabled_icon_opacity",
            slider_values
                .get("kvantum.hacks.disabled_icon_opacity")
                .copied()
                .unwrap_or(100.0),
            0.0, 100.0, 1.0, "%",
        ));
        has_slider_items = true;
    }

    let mut main_col = column![search_field, compat_section].spacing(0).width(Fill);

    if has_slider_items {
        main_col = main_col.push(Space::new().height(12));
        main_col = main_col.push(collapsible_section(
            t("kvantum.hacks.value_title"),
            "kvantum.hacks.values",
            expanded.contains("kvantum.hacks.values"),
            theme::GREEN,
            slider_col.into(),
        ));
    }

    main_col.into()
}

// ── Widgets tab ──────────────────────────────────────────────────────────

/// Kvantum widget section names.
const WIDGET_SECTIONS: &[&str] = &[
    "PanelButtonCommand",
    "PanelButtonTool",
    "DockTitle",
    "PushButton",
    "ToolButton",
    "Tab",
    "TabFrame",
    "TabBarFrame",
    "TreeExpander",
    "HeaderSection",
    "SizeGrip",
    "Toolbar",
    "Statusbar",
    "Scrollbar",
    "ScrollbarGroove",
    "ScrollbarSlider",
    "ProgressbarContents",
    "Progressbar",
    "RadioButton",
    "CheckBox",
    "Focus",
    "GenericFrame",
    "LineEdit",
    "SpinBox",
    "ComboBox",
    "MenuItem",
    "MenuBar",
    "MenuBarItem",
    "Menu",
    "GroupBox",
    "TabWidget",
    "Slider",
    "SliderCursor",
    "Splitter",
    "Window",
    "Dialog",
];

fn widgets_tab<'a>(
    expanded: &HashSet<String>,
    slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let selected_key = "kvantum.widgets.selected";
    let selected = text_input_values
        .get(selected_key)
        .cloned()
        .unwrap_or_default();

    // Left panel: section list
    let mut section_list = column![].spacing(2);

    for &name in WIDGET_SECTIONS {
        let is_selected = selected == name;

        let bg_color = if is_selected {
            Color { a: 0.12, ..theme::GREEN }
        } else {
            Color::TRANSPARENT
        };

        let text_color = if is_selected {
            theme::GREEN
        } else {
            theme::TEXT_ON
        };

        let name_owned = name.to_string();

        let btn = button(
            text(name).size(13).color(text_color),
        )
        .on_press(Message::DropdownChanged {
            key: selected_key.to_string(),
            value: name_owned,
        })
        .padding([8, 16])
        .width(Fill)
        .style(move |_: &Theme, status| {
            let hovered = matches!(status, button::Status::Hovered);
            button::Style {
                background: Some(Background::Color(if hovered {
                    Color { a: 0.08, ..theme::GREEN }
                } else {
                    bg_color
                })),
                border: Border {
                    color: if is_selected { theme::GREEN } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        });

        section_list = section_list.push(btn);
    }

    let left_panel = container(
        scrollable(section_list.width(Fill)).height(Fill),
    )
    .width(200)
    .height(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::SURF)),
        border: Border {
            color: theme::BORDER,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    });

    // Right panel: section detail
    let right_panel: Element<'_, Message> = if !selected.is_empty() {
        widget_section_detail(&selected, expanded, slider_values, text_input_values)
    } else {
        container(
            column![
                text(t("kvantum.widgets.select_placeholder"))
                    .size(14)
                    .color(theme::DIM),
                Space::new().height(8),
                text(t("kvantum.widgets.description"))
                    .size(12)
                    .color(theme::MUTE),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(Fill)
        .height(Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
    };

    row![left_panel, right_panel]
        .spacing(0)
        .height(Fill)
        .into()
}

/// Detail panel for a selected widget section.
/// Shows all actual key=value pairs from the loaded config as editable text fields.
fn widget_section_detail<'a>(
    section_name: &str,
    _expanded: &HashSet<String>,
    _slider_values: &HashMap<String, f32>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let section_label = section_name.to_string();
    let prefix = format!("kvantum.widget.{}", section_name);

    // Inherits dropdown
    let inherits_key = format!("{}.inherits", prefix);
    let inherits_options: Vec<&str> = {
        let mut opts = vec!["(none)"];
        for &s in WIDGET_SECTIONS {
            if s != section_name {
                opts.push(s);
            }
        }
        opts
    };
    let current_inherits = text_input_values
        .get(&inherits_key)
        .map(String::as_str);

    // Collect all keys for this section from text_input_values
    // Keys are stored as "kvantum.widget.<Section>.<property>" = "<value>"
    let key_prefix = format!("{}.", prefix);
    let mut entries: Vec<(&str, &str)> = Vec::new();
    for (k, v) in text_input_values {
        if let Some(prop) = k.strip_prefix(&key_prefix)
            && prop != "inherits" {
                entries.push((prop, v.as_str()));
            }
    }
    entries.sort_by_key(|(k, _)| *k);

    // Build key=value rows
    let mut props_col = column![].spacing(8);
    for (prop, value) in &entries {
        let full_key = format!("{}.{}", prefix, prop);
        let full_key_clone = full_key.clone();
        let prop_label = prop.to_string();
        let val = value.to_string();

        let field_row = row![
            text(prop_label)
                .size(12)
                .color(theme::DIM)
                .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() })
                .width(iced::Length::FillPortion(2)),
            text_input("", &val)
                .on_input(move |v| Message::TextInputChanged {
                    key: full_key_clone.clone(),
                    value: v,
                })
                .size(12)
                .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() })
                .width(iced::Length::FillPortion(3)),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        props_col = props_col.push(field_row);
    }

    if entries.is_empty() {
        props_col = props_col.push(
            text(t("kvantum.widgets.no_properties"))
                .size(12)
                .color(theme::MUTE),
        );
    }

    container(
        scrollable(
            column![
                text(section_label)
                    .size(16)
                    .color(theme::TEXT_ON)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                Space::new().height(12),
                enum_dropdown(
                    t("kvantum.widgets.inherits"),
                    inherits_key,
                    current_inherits,
                    &inherits_options,
                ),
                Space::new().height(16),
                props_col,
            ]
            .padding(16)
            .width(Fill),
        )
        .height(Fill),
    )
    .width(Fill)
    .height(Fill)
    .into()
}
