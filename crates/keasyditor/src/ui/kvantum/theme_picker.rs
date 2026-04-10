use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{KvantumMessage, Message};
use crate::theme;

/// Render a Kvantum theme picker panel. Returns `None` when `visible` is false.
#[allow(dead_code)]
pub fn theme_picker<'a>(
    themes: &[(String, String, bool)], // (name, path, is_system)
    visible: bool,
) -> Option<Element<'a, Message>> {
    if !visible {
        return None;
    }

    let close_btn = button(
        text("\u{2715}").size(14).color(theme::DIM),
    )
    .on_press(Message::Kvantum(KvantumMessage::HideThemePicker))
    .padding([4, 8])
    .style(|_: &Theme, _| button::Style {
        background: None,
        text_color: theme::DIM,
        border: Border {
            color: theme::BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    });

    let header = row![
        text(t("kvantum.themes.title"))
            .size(18)
            .color(theme::TEXT_ON)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
        Space::new().width(Fill),
        close_btn,
    ]
    .align_y(iced::Alignment::Center)
    .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 });

    let mut list = column![].spacing(4);

    if themes.is_empty() {
        list = list.push(
            text(t("kvantum.themes.none_found"))
                .size(12)
                .color(theme::MUTE),
        );
    } else {
        // Separate user and system themes
        let user_themes: Vec<&(String, String, bool)> =
            themes.iter().filter(|(_, _, sys)| !sys).collect();
        let system_themes: Vec<&(String, String, bool)> =
            themes.iter().filter(|(_, _, sys)| *sys).collect();

        if !user_themes.is_empty() {
            list = list.push(
                text(t("kvantum.themes.user"))
                    .size(11)
                    .color(theme::MUTE)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
            );
            list = list.push(Space::new().height(2));
            for (name, path, _) in &user_themes {
                list = list.push(theme_row(name, path, false));
            }
        }

        if !system_themes.is_empty() {
            if !user_themes.is_empty() {
                list = list.push(Space::new().height(8));
            }
            list = list.push(
                text(t("kvantum.themes.system"))
                    .size(11)
                    .color(theme::MUTE)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
            );
            list = list.push(Space::new().height(2));
            for (name, path, _) in &system_themes {
                list = list.push(theme_row(name, path, true));
            }
        }
    }

    let content = column![header, scrollable(list).height(450),].spacing(8);

    let panel = container(content)
        .width(400)
        .padding(16)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: Color {
                    a: 0.4,
                    ..theme::GREEN
                },
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    Some(panel.into())
}

/// A single theme row: dot + name + path, clickable to select.
#[allow(dead_code)]
fn theme_row<'a>(name: &str, path: &str, is_system: bool) -> Element<'a, Message> {
    let dot_color = if is_system { theme::DIM } else { theme::GREEN };
    let name_owned = name.to_string();
    let path_owned = path.to_string();

    let content = row![
        text("\u{25CF}").size(10).color(dot_color),
        column![
            text(name_owned.clone())
                .size(13)
                .color(theme::TEXT_ON),
            text(path_owned)
                .size(10)
                .color(theme::MUTE),
        ]
        .spacing(2),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    button(content)
        .on_press(Message::Kvantum(KvantumMessage::SelectTheme(name_owned)))
        .width(Fill)
        .padding([8, 12])
        .style(|_: &Theme, status| {
            let bg = match status {
                button::Status::Hovered => theme::SURF2,
                _ => theme::SURF,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: theme::TEXT_ON,
                border: Border {
                    color: theme::BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}
