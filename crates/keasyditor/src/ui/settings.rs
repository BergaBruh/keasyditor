use iced::widget::{button, column, container, row, rule, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{Message, SettingsMessage};
use crate::theme;

/// Settings page: engine reload.
pub fn settings_page<'a>(
    reload_status: &'a Option<String>,
) -> Element<'a, Message> {
    let title = text(t("settings.title"))
        .size(28)
        .color(theme::TEXT_ON)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        });

    // Engine reload section
    let reload_header = section_header(t("settings.engine_reload"));

    let klassy_reload = reload_tile(
        t("settings.reload_klassy"),
        t("settings.reload_klassy_description"),
        theme::AMBER,
        Message::Settings(SettingsMessage::ReloadKlassy),
    );

    let kvantum_reload = reload_tile(
        t("settings.reload_kvantum"),
        t("settings.reload_kvantum_description"),
        theme::GREEN,
        Message::Settings(SettingsMessage::ReloadKvantum),
    );

    let mut reload_content =
        column![klassy_reload, rule::horizontal(1), kvantum_reload].spacing(0);

    // Show reload status feedback if present
    if let Some(status) = reload_status {
        let is_error = status.starts_with("Error");
        let status_color = if is_error {
            Color::from_rgb(0.8, 0.3, 0.3)
        } else {
            theme::GREEN
        };
        reload_content = reload_content.push(rule::horizontal(1));
        reload_content = reload_content.push(
            container(
                text(status.as_str())
                    .size(12)
                    .color(status_color),
            )
            .padding([8, 16]),
        );
    }

    let reload_card = container(reload_content)
        .width(Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    let content = column![
        title,
        Space::new().height(32),
        reload_header,
        Space::new().height(12),
        reload_card,
    ];

    scrollable(container(content).padding(32).width(Fill)).into()
}


fn section_header(label: String) -> Element<'static, Message> {
    text(label)
        .size(16)
        .color(theme::DIM)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .into()
}

fn reload_tile(
    label: String,
    subtitle: String,
    accent: Color,
    on_press: Message,
) -> Element<'static, Message> {
    row![
        column![
            text(label).size(14).color(theme::TEXT_ON),
            text(subtitle).size(12).color(theme::MUTE),
        ]
        .spacing(4)
        .width(Fill),
        button(text(t("settings.reload")).size(12).color(accent))
            .on_press(on_press)
            .padding([6, 16])
            .style(move |_: &Theme, _| button::Style {
                background: None,
                text_color: accent,
                border: Border {
                    color: Color { a: 0.3, ..accent },
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }),
    ]
    .align_y(iced::Alignment::Center)
    .padding(16)
    .into()
}

