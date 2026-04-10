use iced::widget::{column, container, pick_list, row, slider, text, toggler};
use iced::{Background, Border, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::Message;
use crate::theme;

/// Format a slider value for display.
fn format_val(v: f32) -> String {
    if v.fract().abs() < 0.01 {
        format!("{}", v as i32)
    } else {
        format!("{:.1}", v)
    }
}

/// A slider field: label + slider + value display (70px wide).
///
/// The text display is derived from `value` automatically.
pub fn range_slider_field<'a>(
    label: impl Into<String>,
    key: impl Into<String>,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    suffix: &'a str,
) -> Element<'a, Message> {
    let label: String = label.into();
    let key_str: String = key.into();
    let key_slider = key_str.clone();
    let key_release = key_str;
    let display = format_val(value);
    let display_with_suffix = if suffix.is_empty() {
        display
    } else {
        format!("{} {}", display, suffix)
    };

    let label_widget = text(label).size(13).color(theme::TEXT_ON);

    let slider_widget = slider(min..=max, value, move |v| Message::SliderChanged {
        key: key_slider.clone(),
        value: v,
    })
    .on_release(Message::SliderReleased(key_release))
    .step(step)
    .width(Fill);

    // Value display box
    let value_box = container(
        text(display_with_suffix)
            .size(12)
            .color(theme::DIM),
    )
    .width(70)
    .padding([6, 8])
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::SURF2)),
        border: Border {
            color: theme::BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    });

    let val_row = row![slider_widget, value_box]
        .spacing(8)
        .align_y(iced::Alignment::Center);

    column![label_widget, val_row].spacing(4).into()
}

/// A toggle field: label + toggler on the right.
pub fn toggle_field<'a>(label: impl Into<String>, key: impl Into<String>, value: bool) -> Element<'a, Message> {
    let label: String = label.into();
    let key_owned: String = key.into();

    row![
        text(label).size(13).color(theme::TEXT_ON).width(Fill),
        toggler(value)
            .on_toggle(move |v| Message::ToggleChanged {
                key: key_owned.clone(),
                value: v,
            })
            .size(20),
    ]
    .spacing(12)
    .align_y(iced::Alignment::Center)
    .into()
}

/// An enum dropdown: label + pick_list using owned Strings.
pub fn enum_dropdown<'a>(
    label: impl Into<String>,
    key: impl Into<String>,
    selected: Option<&'a str>,
    options: &[&str],
) -> Element<'a, Message> {
    let label: String = label.into();
    let key_owned: String = key.into();
    let options_owned: Vec<String> = options.iter().map(|s| s.to_string()).collect();
    let selected_owned: Option<String> = selected.map(|s| s.to_string());

    let list = pick_list(options_owned, selected_owned, move |v: String| {
        Message::DropdownChanged {
            key: key_owned.clone(),
            value: v,
        }
    })
    .width(200)
    .text_size(13)
    .placeholder(t("common.select"));

    row![
        text(label).size(13).color(theme::TEXT_ON).width(Fill),
        list,
    ]
    .spacing(12)
    .align_y(iced::Alignment::Center)
    .into()
}
