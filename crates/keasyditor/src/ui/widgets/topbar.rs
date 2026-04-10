use iced::widget::{button, container, row, rule, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::Message;
use crate::theme;

/// Editor topbar: breadcrumb + file path box + Unsaved badge + action buttons.
#[allow(clippy::too_many_arguments)]
pub fn editor_topbar<'a>(
    crumbs: &[&'a str],
    accent: Color,
    file_path: Option<&'a str>,
    is_dirty: bool,
    can_undo: bool,
    can_redo: bool,
    on_undo: Option<Message>,
    on_redo: Option<Message>,
    on_save_apply: Option<Message>,
    save_flash: bool,
) -> Element<'a, Message> {
    // Breadcrumb
    let mut breadcrumb_row = row![].spacing(0);
    for (i, crumb) in crumbs.iter().enumerate() {
        if i > 0 {
            breadcrumb_row = breadcrumb_row.push(text("  /  ").size(13).color(theme::MUTE));
        }
        let color = if i == 0 { accent } else { theme::DIM };
        breadcrumb_row = breadcrumb_row.push(text(*crumb).size(13).color(color));
    }

    // File path box
    let path_text: String = file_path
        .map(|s| s.to_string())
        .unwrap_or_else(|| t("topbar.no_file"));
    let path_color = if file_path.is_some() {
        theme::DIM
    } else {
        theme::MUTE
    };
    let file_box = container(text(path_text).size(12).color(path_color))
        .height(30)
        .padding([0, 12])
        .max_width(360)
        .align_y(iced::alignment::Vertical::Center)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF2)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        });

    // Unsaved badge
    let unsaved_badge: Element<'a, Message> = if is_dirty {
        container(text(t("topbar.unsaved")).size(11).color(accent))
            .padding([3, 8])
            .style(move |_: &Theme| container::Style {
                background: Some(Background::Color(Color {
                    a: 0.12,
                    ..accent
                })),
                border: Border {
                    color: Color {
                        a: 0.3,
                        ..accent
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        Space::new().into()
    };

    // Center section
    let center = row![file_box, unsaved_badge]
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Action buttons
    let undo_btn = topbar_button(t("topbar.undo"), theme::DIM, can_undo, on_undo, false);
    let redo_btn = topbar_button(t("topbar.redo"), theme::DIM, can_redo, on_redo, false);
    let save_apply_btn = if save_flash {
        topbar_button_flash()
    } else {
        topbar_button(t("topbar.save_apply"), theme::GREEN, is_dirty, on_save_apply, true)
    };

    let actions = row![undo_btn, redo_btn, save_apply_btn].spacing(6);

    let bar_content = row![
        breadcrumb_row,
        Space::new().width(16),
        container(center).width(Fill),
        Space::new().width(16),
        actions,
    ]
    .align_y(iced::Alignment::Center)
    .padding([0, 20]);

    let bar = container(bar_content)
        .height(60)
        .width(Fill)
        .align_y(iced::alignment::Vertical::Center)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            ..Default::default()
        });

    iced::widget::column![bar, rule::horizontal(1)].into()
}

/// A small topbar action button.
fn topbar_button(
    label: String,
    color: Color,
    enabled: bool,
    on_press: Option<Message>,
    is_accent: bool,
) -> Element<'static, Message> {
    let fg = if enabled {
        if is_accent {
            theme::BG
        } else {
            color
        }
    } else {
        Color {
            a: 0.3,
            ..theme::MUTE
        }
    };

    let mut btn = button(text(label).size(12).color(fg))
        .padding([4, 12])
        .style(move |_: &Theme, _| {
            let bg = if !enabled {
                None
            } else if is_accent {
                Some(Background::Color(color))
            } else {
                Some(Background::Color(theme::SURF2))
            };
            let border_color = if !enabled {
                Color::TRANSPARENT
            } else if is_accent {
                color
            } else {
                theme::BORDER
            };
            button::Style {
                background: bg,
                text_color: fg,
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..Default::default()
            }
        });

    if enabled
        && let Some(msg) = on_press {
            btn = btn.on_press(msg);
        }

    btn.into()
}

/// "Saved ✓" flash state — shown for 1 second after successful save.
fn topbar_button_flash() -> Element<'static, Message> {
    let green = theme::GREEN;
    button(
        text("Saved ✓").size(12).color(theme::BG),
    )
    .padding([4, 12])
    .style(move |_: &Theme, _| button::Style {
        background: Some(Background::Color(green)),
        text_color: theme::BG,
        border: Border {
            color: green,
            width: 1.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    })
    .into()
}
