use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::message::Message;
use crate::theme;

/// A collapsible section: header button that toggles visibility of content.
///
/// `section_id` is used to track expanded state in the App's `klassy_expanded`
/// or `kvantum_expanded` HashSet.
pub fn collapsible_section<'a>(
    title: impl Into<String>,
    section_id: &str,
    is_expanded: bool,
    accent: Color,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let title: String = title.into();
    let arrow = if is_expanded { "\u{25BC}" } else { "\u{25B6}" };

    let header = button(
        row![
            text(arrow).size(10).color(theme::DIM),
            Space::new().width(8),
            text(title).size(14).color(theme::TEXT_ON),
        ]
        .align_y(iced::Alignment::Center),
    )
    .on_press(Message::ToggleSection(section_id.to_string()))
    .padding([10, 16])
    .width(Fill)
    .style(move |_: &Theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: Some(Background::Color(if hovered {
                Color {
                    a: 0.06,
                    ..accent
                }
            } else {
                theme::SURF2
            })),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: iced::border::Radius {
                    top_left: 8.0,
                    top_right: 8.0,
                    bottom_right: if is_expanded { 0.0 } else { 8.0 },
                    bottom_left: if is_expanded { 0.0 } else { 8.0 },
                },
            },
            ..Default::default()
        }
    });

    if is_expanded {
        let body = container(content)
            .width(Fill)
            .padding(iced::Padding { top: 12.0, right: 16.0, bottom: 16.0, left: 16.0 })
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(theme::SURF)),
                border: Border {
                    color: theme::BORDER,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_right: 8.0,
                        bottom_left: 8.0,
                    },
                },
                ..Default::default()
            });
        column![header, body].into()
    } else {
        header.into()
    }
}
