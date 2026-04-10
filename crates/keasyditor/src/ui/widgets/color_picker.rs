use std::collections::HashMap;

use iced::widget::{button, canvas, column, container, row, slider, text, text_input, Space};
use iced::{mouse, Background, Border, Color, Element, Event, Fill, Length, Point, Rectangle, Renderer, Size, Theme};

use crate::message::Message;
use crate::theme;
use super::color_utils::{parse_hex, luminance};

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l.clamp(0.0, 1.0), l.clamp(0.0, 1.0), l.clamp(0.0, 1.0));
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let hk = h / 360.0;
    let tr = (hk + 1.0 / 3.0) % 1.0;
    let tg = hk % 1.0;
    let tb = (hk - 1.0 / 3.0 + 1.0) % 1.0;
    let component = |t: f32| -> f32 {
        let v = if t < 1.0 / 6.0 { p + (q - p) * 6.0 * t }
        else if t < 0.5 { q }
        else if t < 2.0 / 3.0 { p + (q - p) * (2.0 / 3.0 - t) * 6.0 }
        else { p };
        v.clamp(0.0, 1.0)
    };
    (component(tr), component(tg), component(tb))
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-6 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if (max - r).abs() < 1e-6 {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < 1e-6 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h * 360.0, s, l)
}

// ── Public color picker field ────────────────────────────────────────────────

pub fn color_picker_field<'a>(
    label: impl Into<String>,
    key_prefix: &str,
    current_hex: &str,
    expanded: bool,
    slider_values: &HashMap<String, f32>,
    text_input_values: &HashMap<String, String>,
) -> Element<'a, Message> {
    let label: String = label.into();
    let color = parse_hex(current_hex);

    let default_r = (color.r * 255.0).round();
    let default_g = (color.g * 255.0).round();
    let default_b = (color.b * 255.0).round();

    let r_key = format!("color.{}.r", key_prefix);
    let g_key = format!("color.{}.g", key_prefix);
    let b_key = format!("color.{}.b", key_prefix);
    let hex_key = format!("color.{}.hex", key_prefix);
    let toggle_key = format!("color.{}", key_prefix);

    let r_val = slider_values.get(&r_key).copied().unwrap_or(default_r);
    let g_val = slider_values.get(&g_key).copied().unwrap_or(default_g);
    let b_val = slider_values.get(&b_key).copied().unwrap_or(default_b);

    let display_hex = rgb_to_hex(r_val as u8, g_val as u8, b_val as u8);
    let swatch_color = Color::from_rgb(
        (r_val / 255.0).clamp(0.0, 1.0),
        (g_val / 255.0).clamp(0.0, 1.0),
        (b_val / 255.0).clamp(0.0, 1.0),
    );

    // -- Header row: label + 28x28 swatch + hex text --
    let toggle_key_clone = toggle_key.clone();
    let swatch = button(Space::new().width(28).height(28))
        .on_press(Message::ToggleSection(toggle_key_clone))
        .padding(0)
        .style(move |_: &Theme, status| {
            let hovered = matches!(status, button::Status::Hovered);
            let border_col = if hovered { theme::TEXT_ON } else { theme::BORDER };
            button::Style {
                background: Some(Background::Color(swatch_color)),
                border: Border { color: border_col, width: 1.0, radius: 4.0.into() },
                ..Default::default()
            }
        });

    let header = row![
        text(label).size(13).color(theme::TEXT_ON).width(Length::FillPortion(3)),
        swatch,
        Space::new().width(12),
        text(display_hex.clone())
            .size(12)
            .color(theme::DIM)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() }),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    if !expanded {
        return header.into();
    }

    // -- Expanded: HSL wheel + RGB sliders + hex input + old/new preview --
    let (h, s, l) = rgb_to_hsl(r_val / 255.0, g_val / 255.0, b_val / 255.0);
    let key_prefix_owned = key_prefix.to_string();

    let wheel = canvas(HslWheel {
        hue: h,
        saturation: s,
        lightness: l,
        key_prefix: key_prefix_owned,
    })
    .width(200)
    .height(200);

    let r_slider = channel_slider("R", &r_key, r_val, Color::from_rgb(0.9, 0.3, 0.3));
    let g_slider = channel_slider("G", &g_key, g_val, Color::from_rgb(0.3, 0.8, 0.4));
    let b_slider = channel_slider("B", &b_key, b_val, Color::from_rgb(0.3, 0.5, 0.9));

    let hex_input_val = text_input_values
        .get(&hex_key)
        .cloned()
        .unwrap_or_else(|| display_hex.clone());
    let hex_key_owned = hex_key.clone();
    let hex_input = row![
        text("#").size(12).color(theme::DIM),
        text_input("RRGGBB", &hex_input_val.trim_start_matches('#'))
            .on_input(move |v| {
                let hex = if v.starts_with('#') { v } else { format!("#{}", v) };
                Message::TextInputChanged { key: hex_key_owned.clone(), value: hex }
            })
            .size(12)
            .width(80)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() }),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    // Old vs New preview bar
    let original_color = parse_hex(current_hex);
    let old_preview = container(
        text("Old").size(10).color(
            if luminance(original_color) < 0.4 { theme::DIM } else { Color::from_rgb(0.2, 0.2, 0.2) }
        ),
    )
    .width(Fill)
    .height(28)
    .padding([0, 8])
    .align_y(iced::alignment::Vertical::Center)
    .style(move |_: &Theme| container::Style {
        background: Some(Background::Color(original_color)),
        border: Border { color: theme::BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    });
    let new_preview = container(
        text("New").size(10).color(
            if luminance(swatch_color) < 0.4 { theme::DIM } else { Color::from_rgb(0.2, 0.2, 0.2) }
        ),
    )
    .width(Fill)
    .height(28)
    .padding([0, 8])
    .align_y(iced::alignment::Vertical::Center)
    .style(move |_: &Theme| container::Style {
        background: Some(Background::Color(swatch_color)),
        border: Border { color: theme::BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    });
    let preview_bar = row![old_preview, new_preview];

    let right_col = column![
        r_slider,
        Space::new().height(4),
        g_slider,
        Space::new().height(4),
        b_slider,
        Space::new().height(8),
        hex_input,
        Space::new().height(8),
        preview_bar,
    ];

    let body = row![
        wheel,
        Space::new().width(12),
        right_col,
    ]
    .align_y(iced::Alignment::Start);

    let expanded_body = container(body)
        .width(Fill)
        .padding([12, 12])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF2)),
            border: Border { color: theme::BORDER, width: 1.0, radius: 6.0.into() },
            ..Default::default()
        });

    column![header, Space::new().height(6), expanded_body]
        .spacing(0)
        .into()
}

// ── Channel slider ───────────────────────────────────────────────────────────

fn channel_slider<'a>(
    label: &'a str,
    key: &str,
    value: f32,
    _accent: Color,
) -> Element<'a, Message> {
    let key_owned = key.to_string();
    let key_release = key.to_string();
    let int_val = value.round() as i32;

    let slider_widget = slider(0.0..=255.0, value, move |v| Message::SliderChanged {
        key: key_owned.clone(),
        value: v,
    })
    .on_release(Message::SliderReleased(key_release))
    .step(1.0)
    .width(Fill);

    let val_display = container(
        text(format!("{}", int_val))
            .size(11)
            .color(theme::DIM)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() }),
    )
    .width(36)
    .padding([2, 4])
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::SURF)),
        border: Border { color: theme::BORDER, width: 1.0, radius: 3.0.into() },
        ..Default::default()
    });

    row![
        text(label).size(12).color(theme::DIM).width(20),
        slider_widget,
        Space::new().width(6),
        val_display,
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center)
    .into()
}

// ── HSL Wheel canvas ─────────────────────────────────────────────────────────

struct HslWheel {
    hue: f32,        // 0-360
    saturation: f32, // 0-1
    lightness: f32,  // 0-1
    key_prefix: String,
}

#[derive(Default)]
struct WheelState {
    dragging: DragTarget,
}

#[derive(Default, Clone, Copy)]
enum DragTarget {
    #[default]
    None,
    HueRing,
    SlSquare,
}

impl canvas::Program<Message> for HslWheel {
    type State = WheelState;

    fn update(
        &self,
        state: &mut WheelState,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let Some(pos) = cursor.position_in(bounds) else {
            return None;
        };

        let cx = bounds.width / 2.0;
        let cy = bounds.height / 2.0;
        let outer_r = cx.min(cy) - 2.0;
        let ring_width = 20.0;
        let inner_r = outer_r - ring_width;
        let sq_half = inner_r * 0.6;

        let dx = pos.x - cx;
        let dy = pos.y - cy;
        let dist = (dx * dx + dy * dy).sqrt();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if dist >= inner_r && dist <= outer_r {
                    state.dragging = DragTarget::HueRing;
                } else if (pos.x - cx).abs() <= sq_half && (pos.y - cy).abs() <= sq_half {
                    state.dragging = DragTarget::SlSquare;
                } else {
                    return None;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let was_dragging = !matches!(state.dragging, DragTarget::None);
                state.dragging = DragTarget::None;
                if was_dragging {
                    return Some(canvas::Action::publish(
                        Message::ColorPickerReleased(self.key_prefix.clone()),
                    ).and_capture());
                }
                return Some(canvas::Action::capture());
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {}
            _ => return None,
        }

        match state.dragging {
            DragTarget::HueRing => {
                let angle = dy.atan2(dx).to_degrees();
                let hue = (angle + 360.0) % 360.0;
                let (r, g, b) = hsl_to_rgb(hue, self.saturation, self.lightness);
                Some(canvas::Action::publish(Message::ColorPickerChanged {
                    key_prefix: self.key_prefix.clone(),
                    r: (r * 255.0).round() as u8,
                    g: (g * 255.0).round() as u8,
                    b: (b * 255.0).round() as u8,
                }).and_capture())
            }
            DragTarget::SlSquare => {
                let s = ((pos.x - (cx - sq_half)) / (sq_half * 2.0)).clamp(0.0, 1.0);
                let l = 1.0 - ((pos.y - (cy - sq_half)) / (sq_half * 2.0)).clamp(0.0, 1.0);
                let (r, g, b) = hsl_to_rgb(self.hue, s, l);
                Some(canvas::Action::publish(Message::ColorPickerChanged {
                    key_prefix: self.key_prefix.clone(),
                    r: (r * 255.0).round() as u8,
                    g: (g * 255.0).round() as u8,
                    b: (b * 255.0).round() as u8,
                }).and_capture())
            }
            DragTarget::None => None,
        }
    }

    fn draw(
        &self,
        _state: &WheelState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let cx = bounds.width / 2.0;
        let cy = bounds.height / 2.0;
        let outer_r = cx.min(cy) - 2.0;
        let ring_width = 20.0;
        let inner_r = outer_r - ring_width;
        let sq_half = inner_r * 0.6;

        // Draw hue ring (360 arcs)
        for deg in 0..360 {
            let angle = (deg as f32).to_radians();
            let (hr, hg, hb) = hsl_to_rgb(deg as f32, 1.0, 0.5);
            let hue_color = Color::from_rgb(hr, hg, hb);

            let x1 = cx + angle.cos() * (inner_r + 1.0);
            let y1 = cy + angle.sin() * (inner_r + 1.0);
            let x2 = cx + angle.cos() * (outer_r - 1.0);
            let y2 = cy + angle.sin() * (outer_r - 1.0);

            frame.stroke(
                &canvas::Path::line(Point::new(x1, y1), Point::new(x2, y2)),
                canvas::Stroke::default().with_color(hue_color).with_width(2.5),
            );
        }

        // Draw SL square
        let sq_x = cx - sq_half;
        let sq_y = cy - sq_half;
        let sq_size = sq_half * 2.0;
        let step = 3.0;
        let mut py = 0.0;
        while py < sq_size {
            let l = 1.0 - py / sq_size;
            let mut px = 0.0;
            while px < sq_size {
                let s = px / sq_size;
                let (cr, cg, cb) = hsl_to_rgb(self.hue, s, l);
                frame.fill(
                    &canvas::Path::rectangle(
                        Point::new(sq_x + px, sq_y + py),
                        Size::new(step + 0.5, step + 0.5),
                    ),
                    Color::from_rgb(cr, cg, cb),
                );
                px += step;
            }
            py += step;
        }

        // SL square border
        frame.stroke(
            &canvas::Path::rectangle(Point::new(sq_x, sq_y), Size::new(sq_size, sq_size)),
            canvas::Stroke::default().with_color(Color::from_rgba(0.4, 0.4, 0.4, 0.6)).with_width(1.0),
        );

        // Hue indicator on ring
        let hue_angle = self.hue.to_radians();
        let mid_r = (inner_r + outer_r) / 2.0;
        let hx = cx + hue_angle.cos() * mid_r;
        let hy = cy + hue_angle.sin() * mid_r;
        frame.stroke(
            &canvas::Path::circle(Point::new(hx, hy), 8.0),
            canvas::Stroke::default().with_color(Color::WHITE).with_width(2.5),
        );

        // SL cursor
        let sl_cx = sq_x + self.saturation * sq_size;
        let sl_cy = sq_y + (1.0 - self.lightness) * sq_size;
        frame.stroke(
            &canvas::Path::circle(Point::new(sl_cx, sl_cy), 6.0),
            canvas::Stroke::default().with_color(Color::WHITE).with_width(2.0),
        );
        frame.stroke(
            &canvas::Path::circle(Point::new(sl_cx, sl_cy), 5.0),
            canvas::Stroke::default().with_color(Color::BLACK).with_width(1.0),
        );

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &WheelState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if !matches!(state.dragging, DragTarget::None) {
            return mouse::Interaction::Grabbing;
        }
        if let Some(pos) = cursor.position_in(bounds) {
            let cx = bounds.width / 2.0;
            let cy = bounds.height / 2.0;
            let outer_r = cx.min(cy) - 2.0;
            let inner_r = outer_r - 20.0;
            let sq_half = inner_r * 0.6;
            let dx = pos.x - cx;
            let dy = pos.y - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if (dist >= inner_r && dist <= outer_r)
                || ((pos.x - cx).abs() <= sq_half && (pos.y - cy).abs() <= sq_half)
            {
                return mouse::Interaction::Pointer;
            }
        }
        mouse::Interaction::default()
    }
}
