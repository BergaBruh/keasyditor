use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{KlassyMessage, Message};
use crate::theme;

/// Render a preset picker panel. Returns `None` when `visible` is false.
#[allow(dead_code)]
pub fn preset_picker<'a>(
    presets: &'a [String],
    visible: bool,
) -> Option<Element<'a, Message>> {
    if !visible {
        return None;
    }

    let close_btn = button(
        text("\u{2715}").size(14).color(theme::DIM),
    )
    .on_press(Message::Klassy(KlassyMessage::HidePresetPicker))
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
        text(t("klassy.presets.title"))
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

    if presets.is_empty() {
        list = list.push(
            text(t("klassy.presets.none_found"))
                .size(12)
                .color(theme::MUTE),
        );
    } else {
        for name in presets {
            let name_owned = name.clone();
            let preset_btn = button(
                row![
                    text("\u{25B6}").size(10).color(theme::AMBER),
                    text(name.as_str())
                        .size(13)
                        .color(theme::TEXT_ON),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            )
            .on_press(Message::Klassy(KlassyMessage::ApplyPreset(name_owned)))
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
            });

            list = list.push(preset_btn);
        }
    }

    let content = column![header, scrollable(list).height(400),].spacing(8);

    let panel = container(content)
        .width(360)
        .padding(16)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: Color {
                    a: 0.4,
                    ..theme::AMBER
                },
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    Some(panel.into())
}
