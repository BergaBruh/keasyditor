use std::cell::Cell;
use std::collections::{HashMap, HashSet};

use iced::widget::{button, canvas, column, container, image, row, rule, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Length, Theme};

use crate::i18n::t;
use crate::message::{KvantumMessage, Message};
use crate::theme;
use crate::ui::kvantum::preview::{kvantum_preview, real_preview_section};
use crate::ui::kvantum::svg_panel::svg_panel;
use crate::ui::kvantum::tabs::kvantum_tab_content;
use crate::ui::widgets::topbar::editor_topbar;

const TAB_COUNT: usize = 5;

fn tab_keys() -> [&'static str; TAB_COUNT] {
    [
        "kvantum.tab.colors",
        "kvantum.tab.general",
        "kvantum.tab.hacks",
        "kvantum.tab.widgets",
        "kvantum.tab.svg",
    ]
}

/// Full Kvantum editor page: topbar + tab bar + scrollable content.
#[allow(clippy::too_many_arguments)]
pub fn kvantum_editor<'a>(
    active_tab: usize,
    is_dirty: bool,
    file_path: Option<&'a str>,
    expanded: &HashSet<String>,
    slider_values: &'a HashMap<String, f32>,
    toggle_values: &'a HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
    svg_content: Option<&'a str>,
    can_undo: bool,
    can_redo: bool,
    loading: bool,
    error: Option<&'a str>,
    save_flash: bool,
    real_preview_handle: Option<&'a image::Handle>,
    real_preview_capturing: bool,
    real_preview_error: Option<&'a str>,
    preview_cache: &'a canvas::Cache,
    preview_cache_key: &'a Cell<u64>,
) -> Element<'a, Message> {
    // If loading, show a loading indicator
    if loading {
        return container(
            column![
                text(t("kvantum.loading"))
                    .size(16)
                    .color(theme::DIM),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(Fill)
        .height(Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into();
    }

    // Empty / error state — no config loaded
    if file_path.is_none() {
        let (icon, msg, detail, color) = if let Some(err) = error {
            ("✗", t("kvantum.error"), err.to_string(), Color::from_rgb(0.75, 0.3, 0.3))
        } else {
            ("◎", t("kvantum.empty.title"), t("kvantum.empty.detail"), theme::MUTE)
        };
        return container(
            column![
                text(icon).size(36).color(color),
                Space::new().height(12),
                text(msg).size(15).color(theme::TEXT_ON),
                Space::new().height(6),
                text(detail).size(12).color(theme::DIM),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(Fill)
        .height(Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into();
    }

    // Topbar
    let topbar = editor_topbar(
        &["Kvantum", "theme.kvconfig"],
        theme::GREEN,
        file_path,
        is_dirty,
        can_undo,
        can_redo,
        Some(Message::Kvantum(KvantumMessage::Undo)),
        Some(Message::Kvantum(KvantumMessage::Redo)),
        Some(Message::Kvantum(KvantumMessage::SaveAndApply)),
        save_flash,
    );

    // Tab bar
    let tab_bar = kvantum_tab_bar(active_tab);

    // Left panel: tab content, flex 3
    // Widgets tab (3) manages its own layout with internal scrolling — don't wrap in scrollable.
    let left_content: Element<'_, Message> = if active_tab == 4 {
        // SVG tab: dual-panel layout with internal scrolling
        container(svg_panel(text_input_values, svg_content))
            .padding(16)
            .width(Fill)
            .height(Fill)
            .into()
    } else if active_tab == 3 {
        // Widgets tab: dual-panel layout, needs height(Fill) directly
        container(kvantum_tab_content(
            active_tab,
            expanded,
            slider_values,
            toggle_values,
            text_input_values,
        ))
        .width(Fill)
        .height(Fill)
        .into()
    } else {
        scrollable(
            container(kvantum_tab_content(
                active_tab,
                expanded,
                slider_values,
                toggle_values,
                text_input_values,
            ))
            .padding(16)
            .width(Fill),
        )
        .height(Fill)
        .into()
    };

    // Vertical divider
    let divider = container(Space::new().width(1).height(Fill)).style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::BORDER)),
        ..Default::default()
    });

    // Right panel: real kvantumpreview screenshot + canvas mock (scrollable),
    // flex 2
    let right_content = scrollable(
        container(
            column![
                real_preview_section(real_preview_handle, real_preview_capturing, real_preview_error),
                Space::new().height(16),
                kvantum_preview(slider_values, toggle_values, text_input_values, preview_cache, preview_cache_key),
            ]
        )
        .padding(16)
        .width(Fill),
    )
    .height(Fill);

    let split_view = row![
        container(left_content)
            .width(Length::FillPortion(3))
            .height(Fill),
        divider,
        container(right_content)
            .width(Length::FillPortion(2))
            .height(Fill),
    ];

    column![topbar, tab_bar, split_view]
        .height(Fill)
        .into()
}

/// Tab bar with 5 tabs (green accent).
fn kvantum_tab_bar(active_tab: usize) -> Element<'static, Message> {
    let mut tabs = row![].spacing(0);

    for (i, key) in tab_keys().iter().enumerate() {
        let is_active = i == active_tab;
        tabs = tabs.push(tab_button(t(key), i, is_active, theme::GREEN));
    }

    tabs = tabs.push(Space::new().width(Fill));

    let bar = container(tabs)
        .width(Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            ..Default::default()
        });

    column![bar, rule::horizontal(1)].into()
}

fn tab_button(
    label: String,
    index: usize,
    is_active: bool,
    accent: Color,
) -> Element<'static, Message> {
    let color = if is_active { accent } else { theme::DIM };

    let tab_content = column![text(label).size(13).color(color)].align_x(iced::Alignment::Center);

    let btn = button(tab_content)
        .on_press(Message::Kvantum(KvantumMessage::TabChanged(index)))
        .padding([10, 20])
        .style(move |_: &Theme, status| {
            let hovered = matches!(status, button::Status::Hovered);
            button::Style {
                background: if hovered {
                    Some(Background::Color(Color {
                        a: 0.04,
                        ..accent
                    }))
                } else {
                    None
                },
                text_color: color,
                border: if is_active {
                    Border {
                        color: Color { a: 0.3, ..accent },
                        width: 1.0,
                        radius: 4.0.into(),
                    }
                } else {
                    Border::default()
                },
                ..Default::default()
            }
        });

    if is_active {
        let indicator = container(Space::new().height(3))
            .width(Fill)
            .style(move |_: &Theme| container::Style {
                background: Some(Background::Color(accent)),
                border: Border {
                    radius: 1.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        column![btn, indicator]
            .width(iced::Length::Shrink)
            .align_x(iced::Alignment::Center)
            .into()
    } else {
        let spacer = Space::new().height(3);
        column![btn, spacer].width(iced::Length::Shrink).into()
    }
}
