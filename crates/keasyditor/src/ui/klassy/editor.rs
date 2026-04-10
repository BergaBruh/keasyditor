use std::collections::{HashMap, HashSet};

use iced::widget::{button, column, container, row, rule, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Length, Theme};

use crate::i18n::t;
use crate::message::{KlassyMessage, Message};
use crate::theme;
use crate::ui::klassy::preview::klassy_preview;
use crate::ui::klassy::tabs::klassy_tab_content;
use crate::ui::widgets::topbar::editor_topbar;

const TAB_COUNT: usize = 6;

/// Full Klassy editor page: topbar + tab bar + split view (3:2).
pub fn klassy_editor<'a>(
    active_tab: usize,
    is_dirty: bool,
    file_path: Option<&'a str>,
    expanded: &HashSet<String>,
    slider_values: &'a HashMap<String, f32>,
    toggle_values: &'a HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
    can_undo: bool,
    can_redo: bool,
    loading: bool,
    error: Option<&'a str>,
    save_flash: bool,
) -> Element<'a, Message> {
    // If loading, show a loading indicator
    if loading {
        return container(
            column![
                text(t("klassy.loading"))
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
            ("✗", t("klassy.error"), err.to_string(), Color::from_rgb(0.75, 0.3, 0.3))
        } else {
            ("◎", t("klassy.empty.title"), t("klassy.empty.detail"), theme::MUTE)
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
        &["Klassy", "klassyrc"],
        theme::AMBER,
        file_path,
        is_dirty,
        can_undo,
        can_redo,
        Some(Message::Klassy(KlassyMessage::Undo)),
        Some(Message::Klassy(KlassyMessage::Redo)),
        Some(Message::Klassy(KlassyMessage::SaveAndApply)),
        save_flash,
    );

    // Tab bar
    let tab_bar = klassy_tab_bar(active_tab);

    // Left panel: tab content (scrollable), flex 3
    let left_content = scrollable(
        container(klassy_tab_content(active_tab, expanded, slider_values, toggle_values, text_input_values))
            .padding(16)
            .width(Fill),
    )
    .height(Fill);

    // Vertical divider
    let divider = container(Space::new().width(1).height(Fill)).style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::BORDER)),
        ..Default::default()
    });

    // Right panel: preview (scrollable), flex 2
    let right_content = scrollable(
        container(klassy_preview(slider_values, toggle_values, text_input_values))
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

/// Tab bar with 6 tabs.
fn klassy_tab_bar(active_tab: usize) -> Element<'static, Message> {
    let tab_labels = [
        t("klassy.tab.buttons"),
        t("klassy.tab.titlebar"),
        t("klassy.tab.window"),
        t("klassy.tab.shadows"),
        t("klassy.tab.animations"),
        t("klassy.tab.advanced"),
    ];

    let mut tabs = row![].spacing(0);

    for i in 0..TAB_COUNT {
        let is_active = i == active_tab;
        tabs = tabs.push(tab_button(tab_labels[i].clone(), i, is_active, theme::AMBER));
    }

    // Spacer pushes "Presets" button to the right
    tabs = tabs.push(Space::new().width(Fill));
    tabs = tabs.push(
        button(text(t("klassy.presets.title")).size(12).color(theme::DIM))
            .padding([8, 16])
            .style(|_: &Theme, _| button::Style {
                background: None,
                text_color: theme::DIM,
                ..Default::default()
            }),
    );

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

    let tab_label = column![text(label).size(13).color(color)]
        .align_x(iced::Alignment::Center);

    let btn = button(tab_label)
        .on_press(Message::Klassy(KlassyMessage::TabChanged(index)))
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

    // Active tab gets a bottom indicator bar (Shrink width to match button)
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
