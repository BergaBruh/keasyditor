use std::collections::HashMap;

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{Message, SvgMessage};
use crate::theme;
use crate::ui::widgets::color_utils::parse_hex;

/// The SVG element editor for the Kvantum editor.
///
/// Left panel: widget type groups discovered from the actual SVG.
/// Right panel: element list with editable fill/stroke fields.
pub fn svg_panel<'a>(
    text_input_values: &'a HashMap<String, String>,
    svg_content: Option<&'a str>,
) -> Element<'a, Message> {
    let Some(svg) = svg_content else {
        let import_btn = button(
            text(t("kvantum.svg.import")).size(13).color(theme::GREEN),
        )
        .on_press(Message::Svg(SvgMessage::ImportSvg))
        .padding([8, 20])
        .style(|_: &Theme, _| button::Style {
            background: Some(Background::Color(theme::GREEN)),
            text_color: theme::BG,
            border: Border { radius: 6.0.into(), ..Default::default() },
            ..Default::default()
        });

        return container(
            column![
                text(t("kvantum.svg.no_file_title")).size(14).color(theme::DIM),
                Space::new().height(8),
                text(t("kvantum.svg.no_file_description"))
                    .size(12)
                    .color(theme::MUTE),
                Space::new().height(16),
                import_btn,
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(Fill)
        .height(Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into();
    };

    // Discover element groups from the actual SVG
    let catalog = keasyditor_core::svg::catalog_elements(svg);

    static EMPTY: String = String::new();
    let selected_type_key = "kvantum.svg.selected_type";
    let selected_type = text_input_values
        .get(selected_type_key)
        .unwrap_or(&EMPTY);

    // Header with import button
    let import_btn = button(
        text(t("kvantum.svg.import")).size(12).color(theme::GREEN),
    )
    .on_press(Message::Svg(SvgMessage::ImportSvg))
    .padding([6, 16])
    .style(|_: &Theme, _| button::Style {
        background: None,
        text_color: theme::GREEN,
        border: Border {
            color: Color { a: 0.4, ..theme::GREEN },
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    });

    let header = column![
        row![
            text(t("kvantum.svg.editor_title"))
                .size(16)
                .color(theme::TEXT_ON)
                .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
            Space::new().width(Fill),
            import_btn,
        ].align_y(iced::Alignment::Center),
        Space::new().height(4),
        text(t("kvantum.svg.editor_description"))
            .size(12)
            .color(theme::DIM),
        Space::new().height(16),
    ];

    // Left panel: widget type list from catalog
    let mut type_names: Vec<String> = catalog.keys().cloned().collect();
    type_names.sort();

    let mut type_list = column![].spacing(2);
    for type_name in &type_names {
        let is_selected = selected_type == type_name.as_str();
        let elements = catalog.get(type_name.as_str()).map(|v| v.len()).unwrap_or(0);
        let type_owned = type_name.clone();

        let bg_color = if is_selected {
            Color { a: 0.12, ..theme::GREEN }
        } else {
            Color::TRANSPARENT
        };
        let text_color = if is_selected { theme::GREEN } else { theme::TEXT_ON };

        let display_name = type_name.clone();
        let btn = button(
            row![
                text(display_name).size(13).color(text_color).width(Fill),
                text(format!("{}", elements)).size(11).color(theme::MUTE),
            ]
            .align_y(iced::Alignment::Center),
        )
        .on_press(Message::DropdownChanged {
            key: selected_type_key.to_string(),
            value: type_owned,
        })
        .padding([8, 12])
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

        type_list = type_list.push(btn);
    }

    let left_panel = container(
        scrollable(type_list.width(Fill)).height(Fill),
    )
    .width(180)
    .height(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::SURF)),
        border: Border { color: theme::BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    });

    // Right panel: element list with fill/stroke editing
    let right_panel: Element<'_, Message> = if !selected_type.is_empty() {
        let elements = catalog
            .get(selected_type.as_str())
            .cloned()
            .unwrap_or_default();

        if elements.is_empty() {
            container(
                text(t("kvantum.svg.no_elements")).size(13).color(theme::MUTE),
            )
            .width(Fill)
            .height(Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .into()
        } else {
            let mut elem_col = column![].spacing(6);

            for element_id in &elements {
                let fill = keasyditor_core::svg::get_fill_color(svg, element_id);
                let stroke = keasyditor_core::svg::get_stroke_color(svg, element_id);

                elem_col = elem_col.push(svg_element_row(element_id, fill.as_deref(), stroke.as_deref()));
            }

            container(
                column![
                    text(selected_type.as_str())
                        .size(14)
                        .color(theme::TEXT_ON)
                        .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                    Space::new().height(4),
                    text(format!("{} {}", elements.len(), t("kvantum.svg.elements_suffix")))
                        .size(11)
                        .color(theme::MUTE),
                    Space::new().height(12),
                    scrollable(elem_col.width(Fill)).height(Fill),
                ]
                .padding(12)
                .width(Fill),
            )
            .width(Fill)
            .height(Fill)
            .into()
        }
    } else {
        container(
            column![
                text(t("kvantum.svg.select_type")).size(14).color(theme::DIM),
                Space::new().height(8),
                text(t("kvantum.svg.select_type_description"))
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

    column![
        header,
        row![left_panel, Space::new().width(8), right_panel]
            .spacing(0)
            .height(Fill),
    ]
    .width(Fill)
    .height(Fill)
    .into()
}

/// A single SVG element row: id + color swatch + fill input + stroke input.
fn svg_element_row<'a>(
    element_id: &str,
    fill: Option<&str>,
    stroke: Option<&str>,
) -> Element<'a, Message> {
    let id_owned = element_id.to_string();

    // Fill swatch + input
    let fill_val = fill.unwrap_or("").to_string();
    let fill_color = if fill_val.starts_with('#') { parse_hex(&fill_val) } else { Color::TRANSPARENT };
    let fill_id = id_owned.clone();

    let fill_swatch = container(Space::new().width(16).height(16))
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(fill_color)),
            border: Border { color: theme::BORDER, width: 1.0, radius: 3.0.into() },
            ..Default::default()
        });

    let fill_field = row![
        text(t("kvantum.svg.fill")).size(10).color(theme::MUTE).width(32),
        fill_swatch,
        Space::new().width(4),
        text_input("none", &fill_val)
            .on_input(move |v| Message::Svg(SvgMessage::UpdateFill {
                element_id: fill_id.clone(),
                color: v,
            }))
            .size(11)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() })
            .width(90),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    // Stroke swatch + input
    let stroke_val = stroke.unwrap_or("").to_string();
    let stroke_color = if stroke_val.starts_with('#') { parse_hex(&stroke_val) } else { Color::TRANSPARENT };
    let stroke_id = id_owned.clone();

    let stroke_swatch = container(Space::new().width(16).height(16))
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(stroke_color)),
            border: Border { color: theme::BORDER, width: 1.0, radius: 3.0.into() },
            ..Default::default()
        });

    let stroke_field = row![
        text(t("kvantum.svg.stroke")).size(10).color(theme::MUTE).width(32),
        stroke_swatch,
        Space::new().width(4),
        text_input("none", &stroke_val)
            .on_input(move |v| Message::Svg(SvgMessage::UpdateStroke {
                element_id: stroke_id.clone(),
                color: v,
            }))
            .size(11)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() })
            .width(90),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    // Element row: id label + fill + stroke
    container(
        row![
            text(id_owned)
                .size(12)
                .color(theme::TEXT_ON)
                .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() })
                .width(Fill),
            fill_field,
            Space::new().width(8),
            stroke_field,
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
    )
    .padding([6, 8])
    .width(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::SURF)),
        border: Border { color: theme::BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .into()
}
