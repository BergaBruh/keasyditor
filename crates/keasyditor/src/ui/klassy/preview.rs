use std::collections::HashMap;

use iced::widget::{canvas, column, container, text, Space};
use iced::{mouse, Color, Element, Fill, Point, Rectangle, Renderer, Size, Theme};

use crate::i18n::t;
use crate::message::Message;
use crate::theme as pal;
use crate::ui::widgets::canvas_utils::{rounded_rect, rounded_rect_top};
use crate::ui::widgets::color_utils::read_color;

// ── Local enums for preview rendering ────────────────────────────────────────

#[derive(Clone, Copy, Default)]
enum BtnShape {
    #[default]
    SmallCircle,
    SmallSquare,
    FullHeightRect,
    FullHeightRoundedRect,
    IntegratedRoundedRect,
}

#[derive(Clone, Copy, Default)]
enum BgColorScheme {
    #[default]
    TitleBarText,
    Accent,
    AccentTrafficLights,
    AccentWithNegativeClose,
    TitleBarTextNegativeClose,
}

#[derive(Clone, Copy, Default)]
enum OutlineStyle {
    #[default]
    StyleNone,
    AccentColor,
    CustomColor,
    AccentWithContrast,
}

#[derive(Clone, Copy, Default)]
enum TitleAlign {
    #[default]
    Left,
    Center,
    Right,
}

fn slider(vals: &HashMap<String, f32>, key: &str, default: f32) -> f32 {
    vals.get(key).copied().unwrap_or(default)
}

fn toggle(vals: &HashMap<String, bool>, key: &str, default: bool) -> bool {
    vals.get(key).copied().unwrap_or(default)
}

fn border_size_px(val: Option<&String>) -> (f32, bool) {
    match val.map(String::as_str) {
        Some("Tiny") => (1.0, true),
        Some("Normal") => (2.0, true),
        Some("Large") => (4.0, true),
        Some("VeryLarge") => (6.0, true),
        Some("Huge") => (8.0, true),
        Some("VeryHuge") => (10.0, true),
        Some("Oversized") => (12.0, true),
        Some("NoSides") => (2.0, false), // bottom only
        _ => (0.0, true),
    }
}

// ── Public entry point ───────────────────────────────────────────────────────

pub fn klassy_preview<'a>(
    slider_values: &'a HashMap<String, f32>,
    toggle_values: &'a HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    // Extract all values once
    let corner_radius = slider(slider_values, "klassy.corner_radius", 10.0);
    let shadow_strength = slider(slider_values, "klassy.shadow_strength", 128.0);
    let shadow_size = slider(slider_values, "klassy.shadow_size", 50.0);
    let shadow_color = read_color("klassy.shadow_color", text_input_values, slider_values, Color::BLACK);

    let tb_opacity_active = (slider(slider_values, "klassy.titlebar_opacity", 100.0) / 100.0).clamp(0.0, 1.0);
    let tb_opacity_inactive = (slider(slider_values, "klassy.titlebar_opacity_inactive", 100.0) / 100.0).clamp(0.0, 1.0);
    let tb_top_margin = slider(slider_values, "klassy.titlebar_top_margin", 0.0);
    let tb_bottom_margin = slider(slider_values, "klassy.titlebar_bottom_margin", 0.0);
    let tb_side_padding = slider(slider_values, "klassy.titlebar_side_padding", 12.0);

    let titlebar_separator = toggle(toggle_values, "klassy.titlebar_separator", false);
    let active_highlight = toggle(toggle_values, "klassy.active_highlight", false);

    let button_spacing = slider(slider_values, "klassy.button_spacing_right", 8.0);
    let bg_opacity_active = (slider(slider_values, "klassy.bg_opacity_active", 15.0) / 100.0).clamp(0.0, 1.0);
    let bg_opacity_inactive = (slider(slider_values, "klassy.bg_opacity_inactive", 15.0) / 100.0).clamp(0.0, 1.0);

    let button_shape = match text_input_values.get("klassy.button_shape").map(String::as_str) {
        Some("SmallSquare") => BtnShape::SmallSquare,
        Some("FullHeightRectangle") => BtnShape::FullHeightRect,
        Some("FullHeightRoundedRectangle") => BtnShape::FullHeightRoundedRect,
        Some("IntegratedRoundedRectangle") => BtnShape::IntegratedRoundedRect,
        _ => BtnShape::SmallCircle,
    };

    let bg_scheme = |key: &str| match text_input_values.get(key).map(String::as_str) {
        Some("Accent") => BgColorScheme::Accent,
        Some("AccentTrafficLights") => BgColorScheme::AccentTrafficLights,
        Some("AccentWithNegativeClose") => BgColorScheme::AccentWithNegativeClose,
        Some("TitleBarTextNegativeClose") => BgColorScheme::TitleBarTextNegativeClose,
        _ => BgColorScheme::TitleBarText,
    };
    let bg_scheme_active = bg_scheme("klassy.bg_colors_active");
    let bg_scheme_inactive = bg_scheme("klassy.bg_colors_inactive");

    let close_color = read_color("klassy.close_icon_color", text_input_values, slider_values,
        Color::from_rgb(1.0, 0.33, 0.33));
    let min_color = read_color("klassy.minimize_icon_color", text_input_values, slider_values,
        Color::from_rgb(0.93, 0.93, 0.93));
    let max_color = read_color("klassy.maximize_icon_color", text_input_values, slider_values,
        Color::from_rgb(0.93, 0.93, 0.93));

    let outline_style = |key: &str| match text_input_values.get(key).map(String::as_str) {
        Some("WindowOutlineAccentColor") => OutlineStyle::AccentColor,
        Some("WindowOutlineCustomColor") => OutlineStyle::CustomColor,
        Some("WindowOutlineAccentWithContrast") => OutlineStyle::AccentWithContrast,
        _ => OutlineStyle::StyleNone,
    };
    let outline_active = outline_style("klassy.outline_active");
    let outline_inactive = outline_style("klassy.outline_inactive");

    let outline_color_active = read_color("klassy.outline_color_active", text_input_values, slider_values,
        Color::from_rgb(0.5, 0.5, 0.5));
    let outline_color_inactive = read_color("klassy.outline_color_inactive", text_input_values, slider_values,
        Color::from_rgb(0.5, 0.5, 0.5));

    let title_alignment = match text_input_values.get("klassy.title_alignment").map(String::as_str) {
        Some("Center") | Some("AlignCenter") | Some("AlignCenterFullWidth") => TitleAlign::Center,
        Some("Right") | Some("AlignRight") => TitleAlign::Right,
        _ => TitleAlign::Left,
    };

    let (bdr_size, bdr_sides) = border_size_px(text_input_values.get("klassy.kwin_border_size"));

    // Build preview structs
    let make_preview = |is_active: bool| WindowPreview {
        corner_radius,
        shadow_alpha: (shadow_strength / 255.0).clamp(0.0, 1.0),
        shadow_size,
        shadow_color,
        titlebar_opacity: if is_active { tb_opacity_active } else { tb_opacity_inactive },
        titlebar_top_margin: tb_top_margin,
        titlebar_bottom_margin: tb_bottom_margin,
        titlebar_side_padding: tb_side_padding,
        titlebar_separator,
        active_highlight,
        title_alignment,
        button_shape,
        button_spacing,
        bg_opacity: if is_active { bg_opacity_active } else { bg_opacity_inactive },
        bg_color_scheme: if is_active { bg_scheme_active } else { bg_scheme_inactive },
        close_icon_color: close_color,
        minimize_icon_color: min_color,
        maximize_icon_color: max_color,
        outline_style: if is_active { outline_active } else { outline_inactive },
        outline_color: if is_active { outline_color_active } else { outline_color_inactive },
        border_size: bdr_size,
        border_sides: bdr_sides,
        is_active,
    };

    let active_label = container(text(t("klassy.preview.active")).size(12).color(pal::DIM))
        .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 });

    let inactive_label = container(text(t("klassy.preview.inactive")).size(12).color(pal::DIM))
        .padding(iced::Padding { top: 16.0, right: 0.0, bottom: 8.0, left: 0.0 });

    column![
        text(t("klassy.preview.title"))
            .size(16)
            .color(pal::TEXT_ON)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        Space::new().height(12),
        active_label,
        canvas(make_preview(true)).width(Fill).height(180),
        inactive_label,
        canvas(make_preview(false)).width(Fill).height(180),
    ]
    .into()
}

// ── WindowPreview struct ─────────────────────────────────────────────────────

struct WindowPreview {
    corner_radius: f32,
    shadow_alpha: f32,
    shadow_size: f32,
    shadow_color: Color,
    titlebar_opacity: f32,
    titlebar_top_margin: f32,
    titlebar_bottom_margin: f32,
    titlebar_side_padding: f32,
    titlebar_separator: bool,
    active_highlight: bool,
    title_alignment: TitleAlign,
    button_shape: BtnShape,
    button_spacing: f32,
    bg_opacity: f32,
    bg_color_scheme: BgColorScheme,
    close_icon_color: Color,
    minimize_icon_color: Color,
    maximize_icon_color: Color,
    outline_style: OutlineStyle,
    outline_color: Color,
    border_size: f32,
    border_sides: bool,
    is_active: bool,
}

impl<Message> canvas::Program<Message> for WindowPreview {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let margin = 16.0;
        let x = margin;
        let y = margin;
        let w = bounds.width - 2.0 * margin;
        let h = bounds.height - 2.0 * margin;
        let r = self.corner_radius;
        let tb_base = 32.0;
        let tb_h = self.titlebar_top_margin + tb_base + self.titlebar_bottom_margin;

        draw_shadow(&mut frame, x, y, w, h, r, self);
        draw_body(&mut frame, x, y, w, h, r, self.is_active);
        draw_outline(&mut frame, x, y, w, h, r, self);
        draw_titlebar(&mut frame, x, y, w, tb_h, r, self);
        if self.is_active && self.active_highlight {
            draw_active_highlight(&mut frame, x, y, w, r);
        }
        if self.titlebar_separator {
            draw_separator(&mut frame, x, y, w, tb_h);
        }
        draw_title_text(&mut frame, x, y, w, tb_h, self);
        draw_buttons(&mut frame, x, y, w, self);
        draw_border(&mut frame, x, y, w, h, self);
        draw_content_lines(&mut frame, x, y, w, h, tb_h);

        vec![frame.into_geometry()]
    }
}

// ── Drawing functions ────────────────────────────────────────────────────────

fn draw_shadow(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, h: f32, r: f32,
    p: &WindowPreview,
) {
    let layers = ((p.shadow_size / 15.0).ceil() as usize).clamp(1, 6);
    for i in 0..layers {
        let t = (i as f32 + 1.0) / layers as f32;
        let spread = t * p.shadow_size * 0.06;
        let alpha = p.shadow_alpha * (1.0 - t * 0.6) * 0.3;
        let c = Color { a: alpha, ..p.shadow_color };
        frame.fill(
            &rounded_rect(x + spread * 0.3, y + spread * 0.8, w + spread * 0.6, h + spread * 0.4, r),
            c,
        );
    }
}

fn draw_body(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, h: f32, r: f32,
    is_active: bool,
) {
    let color = if is_active {
        Color::from_rgb(0.17, 0.17, 0.17)
    } else {
        Color::from_rgb(0.23, 0.23, 0.23)
    };
    frame.fill(&rounded_rect(x, y, w, h, r), color);
}

fn draw_outline(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, h: f32, r: f32,
    p: &WindowPreview,
) {
    let color = match p.outline_style {
        OutlineStyle::StyleNone => return,
        OutlineStyle::AccentColor => Color { a: 0.6, ..pal::AMBER },
        OutlineStyle::CustomColor => p.outline_color,
        OutlineStyle::AccentWithContrast => Color { a: 0.85, ..pal::AMBER },
    };
    frame.stroke(
        &rounded_rect(x, y, w, h, r),
        canvas::Stroke::default().with_color(color).with_width(1.0),
    );
}

fn draw_titlebar(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, tb_h: f32, r: f32,
    p: &WindowPreview,
) {
    let color = if p.is_active {
        Color::from_rgba(0.24, 0.24, 0.24, p.titlebar_opacity)
    } else {
        Color::from_rgba(0.20, 0.20, 0.20, p.titlebar_opacity)
    };
    // Clip to window bounds so titlebar doesn't overflow rounded corners
    frame.with_clip(Rectangle::new(Point::new(x, y), Size::new(w, tb_h)), |frame| {
        frame.fill(&rounded_rect_top(x, y, w, tb_h, r), color);
    });
}

fn draw_active_highlight(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, r: f32,
) {
    frame.with_clip(
        Rectangle::new(Point::new(x, y), Size::new(w, 2.0)),
        |frame| {
            frame.fill(
                &rounded_rect(x, y, w, 20.0, r),
                Color { a: 0.7, ..pal::AMBER },
            );
        },
    );
}

fn draw_separator(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, tb_h: f32,
) {
    frame.stroke(
        &canvas::Path::line(
            Point::new(x + 1.0, y + tb_h),
            Point::new(x + w - 1.0, y + tb_h),
        ),
        canvas::Stroke::default()
            .with_color(Color::from_rgba(0.5, 0.5, 0.5, 0.25))
            .with_width(1.0),
    );
}

fn draw_title_text(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, tb_h: f32,
    p: &WindowPreview,
) {
    let title = if p.is_active { t("klassy.preview.active") } else { t("klassy.preview.inactive") };
    let color = if p.is_active {
        Color::from_rgb(0.93, 0.93, 0.93)
    } else {
        Color::from_rgb(0.6, 0.6, 0.6)
    };

    let (pos_x, align_x) = match p.title_alignment {
        TitleAlign::Left => (x + p.titlebar_side_padding, iced::alignment::Horizontal::Left),
        TitleAlign::Center => (x + w / 2.0, iced::alignment::Horizontal::Center),
        TitleAlign::Right => (x + w - p.titlebar_side_padding, iced::alignment::Horizontal::Right),
    };

    frame.fill_text(canvas::Text {
        content: title.to_string(),
        position: Point::new(pos_x, y + tb_h / 2.0),
        color,
        size: iced::Pixels(12.0),
        align_x: align_x.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });
}

fn draw_buttons(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WindowPreview,
) {
    let tb_base = 32.0;
    let btn_area_top = y + p.titlebar_top_margin;
    let btn_area_h = tb_base;
    let btn_cy = btn_area_top + btn_area_h / 2.0;
    let right_edge = x + w - p.titlebar_side_padding;

    // Resolve background colors: [minimize, maximize, close]
    let bg_colors = resolve_button_bg_colors(p.bg_color_scheme, p.bg_opacity, p.is_active);
    let icon_colors = [p.minimize_icon_color, p.maximize_icon_color, p.close_icon_color];

    match p.button_shape {
        BtnShape::SmallCircle => {
            draw_small_circle_buttons(frame, right_edge, btn_cy, p.button_spacing, &bg_colors, &icon_colors);
        }
        BtnShape::SmallSquare => {
            draw_small_square_buttons(frame, right_edge, btn_cy, p.button_spacing, &bg_colors, &icon_colors);
        }
        BtnShape::FullHeightRect => {
            draw_full_height_buttons(frame, right_edge, btn_area_top, btn_area_h, 0.0, &bg_colors, &icon_colors);
        }
        BtnShape::FullHeightRoundedRect => {
            draw_full_height_buttons(frame, right_edge, btn_area_top, btn_area_h, 4.0, &bg_colors, &icon_colors);
        }
        BtnShape::IntegratedRoundedRect => {
            draw_integrated_buttons(frame, x, y, w, right_edge, btn_area_top, btn_area_h, p.corner_radius, &bg_colors, &icon_colors);
        }
    }
}

fn resolve_button_bg_colors(scheme: BgColorScheme, opacity: f32, is_active: bool) -> [Color; 3] {
    let tb_text = if is_active {
        Color::from_rgb(0.93, 0.93, 0.93)
    } else {
        Color::from_rgb(0.6, 0.6, 0.6)
    };
    let accent = pal::AMBER;
    let red = Color::from_rgb(0.9, 0.22, 0.22);
    let green = Color::from_rgb(0.35, 0.75, 0.35);
    let blue = Color::from_rgb(0.35, 0.55, 0.85);

    let [min_c, max_c, close_c] = match scheme {
        BgColorScheme::TitleBarText => [tb_text, tb_text, tb_text],
        BgColorScheme::Accent => [accent, accent, accent],
        BgColorScheme::AccentTrafficLights => [blue, green, red],
        BgColorScheme::AccentWithNegativeClose => [accent, accent, red],
        BgColorScheme::TitleBarTextNegativeClose => [tb_text, tb_text, red],
    };

    [
        Color { a: opacity, ..min_c },
        Color { a: opacity, ..max_c },
        Color { a: opacity, ..close_c },
    ]
}

// ── Button shape renderers ───────────────────────────────────────────────────

fn draw_small_circle_buttons(
    frame: &mut canvas::Frame,
    right_edge: f32,
    cy: f32,
    spacing: f32,
    bg_colors: &[Color; 3],
    icon_colors: &[Color; 3],
) {
    let r = 7.0;
    let total_w = 3.0 * r * 2.0 + 2.0 * spacing;
    let mut bx = right_edge - total_w;

    for i in 0..3 {
        let cx = bx + r;
        frame.fill(&canvas::Path::circle(Point::new(cx, cy), r), bg_colors[i]);
        draw_button_icon(frame, cx, cy, r * 0.4, i, icon_colors[i]);
        bx += r * 2.0 + spacing;
    }
}

fn draw_small_square_buttons(
    frame: &mut canvas::Frame,
    right_edge: f32,
    cy: f32,
    spacing: f32,
    bg_colors: &[Color; 3],
    icon_colors: &[Color; 3],
) {
    let size = 14.0;
    let total_w = 3.0 * size + 2.0 * spacing;
    let mut bx = right_edge - total_w;

    for i in 0..3 {
        let cx = bx + size / 2.0;
        frame.fill(
            &canvas::Path::rectangle(Point::new(bx, cy - size / 2.0), Size::new(size, size)),
            bg_colors[i],
        );
        draw_button_icon(frame, cx, cy, size * 0.28, i, icon_colors[i]);
        bx += size + spacing;
    }
}

fn draw_full_height_buttons(
    frame: &mut canvas::Frame,
    right_edge: f32,
    top: f32,
    height: f32,
    corner_r: f32,
    bg_colors: &[Color; 3],
    icon_colors: &[Color; 3],
) {
    let btn_w = 28.0;
    let gap = if corner_r > 0.0 { 2.0 } else { 0.0 };
    let total_w = 3.0 * btn_w + 2.0 * gap;
    let mut bx = right_edge - total_w;
    let cy = top + height / 2.0;

    for i in 0..3 {
        if corner_r > 0.0 {
            frame.fill(&rounded_rect(bx, top, btn_w, height, corner_r), bg_colors[i]);
        } else {
            frame.fill(
                &canvas::Path::rectangle(Point::new(bx, top), Size::new(btn_w, height)),
                bg_colors[i],
            );
            // Separator line between buttons (not after last)
            if i < 2 {
                frame.stroke(
                    &canvas::Path::line(
                        Point::new(bx + btn_w, top + 4.0),
                        Point::new(bx + btn_w, top + height - 4.0),
                    ),
                    canvas::Stroke::default()
                        .with_color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))
                        .with_width(1.0),
                );
            }
        }
        let cx = bx + btn_w / 2.0;
        draw_button_icon(frame, cx, cy, 5.0, i, icon_colors[i]);
        bx += btn_w + gap;
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_integrated_buttons(
    frame: &mut canvas::Frame,
    win_x: f32, win_y: f32, win_w: f32,
    _right_edge: f32,
    top: f32,
    height: f32,
    win_corner_r: f32,
    bg_colors: &[Color; 3],
    icon_colors: &[Color; 3],
) {
    let btn_w = 28.0;
    let total_w = 3.0 * btn_w;
    let cy = top + height / 2.0;

    let group_right = win_x + win_w;
    let group_left = group_right - total_w;

    // Fill individual button backgrounds with a subtle tint difference
    for i in 0..3 {
        let bx = group_left + i as f32 * btn_w;
        // Use clipping to get the window corner rounding on the last button
        if i == 2 {
            frame.with_clip(
                Rectangle::new(Point::new(bx, win_y), Size::new(btn_w + 1.0, height + (top - win_y))),
                |frame| {
                    frame.fill(&rounded_rect(bx, win_y, btn_w, height + (top - win_y), win_corner_r), bg_colors[i]);
                },
            );
        } else {
            frame.fill(
                &canvas::Path::rectangle(Point::new(bx, top), Size::new(btn_w, height)),
                bg_colors[i],
            );
        }

        // Separator lines between buttons
        if i < 2 {
            frame.stroke(
                &canvas::Path::line(
                    Point::new(bx + btn_w, top + 4.0),
                    Point::new(bx + btn_w, top + height - 4.0),
                ),
                canvas::Stroke::default()
                    .with_color(Color::from_rgba(0.5, 0.5, 0.5, 0.15))
                    .with_width(1.0),
            );
        }

        let cx = bx + btn_w / 2.0;
        draw_button_icon(frame, cx, cy, 5.0, i, icon_colors[i]);
    }
}

/// Draw a button icon: 0=minimize, 1=maximize, 2=close.
fn draw_button_icon(
    frame: &mut canvas::Frame,
    cx: f32, cy: f32,
    s: f32,
    index: usize,
    color: Color,
) {
    let stroke = canvas::Stroke::default().with_color(color).with_width(1.5);
    match index {
        0 => {
            // Minimize: horizontal line
            frame.stroke(
                &canvas::Path::line(Point::new(cx - s, cy), Point::new(cx + s, cy)),
                stroke,
            );
        }
        1 => {
            // Maximize: square outline
            frame.stroke(
                &canvas::Path::rectangle(
                    Point::new(cx - s, cy - s),
                    Size::new(s * 2.0, s * 2.0),
                ),
                stroke,
            );
        }
        2 => {
            // Close: X
            frame.stroke(
                &canvas::Path::line(Point::new(cx - s, cy - s), Point::new(cx + s, cy + s)),
                stroke,
            );
            frame.stroke(
                &canvas::Path::line(Point::new(cx + s, cy - s), Point::new(cx - s, cy + s)),
                stroke,
            );
        }
        _ => {}
    }
}

fn draw_border(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, h: f32,
    p: &WindowPreview,
) {
    if p.border_size <= 0.0 {
        return;
    }
    let color = Color::from_rgba(0.45, 0.45, 0.45, 0.45);
    // Bottom
    frame.fill(
        &canvas::Path::rectangle(
            Point::new(x, y + h - p.border_size),
            Size::new(w, p.border_size),
        ),
        color,
    );
    // Sides
    if p.border_sides {
        frame.fill(
            &canvas::Path::rectangle(Point::new(x, y), Size::new(p.border_size, h)),
            color,
        );
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(x + w - p.border_size, y),
                Size::new(p.border_size, h),
            ),
            color,
        );
    }
}

fn draw_content_lines(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32, h: f32, tb_h: f32,
) {
    let start_y = y + tb_h + 12.0;
    let line_color = Color::from_rgba(0.3, 0.3, 0.3, 0.5);
    let mut ly = start_y;
    let mut idx: usize = 0;
    while ly < y + h - 10.0 {
        let frac = match idx % 5 {
            0 => 0.85,
            1 => 0.6,
            2 => 0.75,
            3 => 0.5,
            _ => 0.65,
        };
        let line_w = (w - 24.0) * frac;
        frame.stroke(
            &canvas::Path::line(
                Point::new(x + 12.0, ly),
                Point::new(x + 12.0 + line_w, ly),
            ),
            canvas::Stroke::default().with_color(line_color).with_width(1.0),
        );
        ly += 12.0;
        idx += 1;
    }
}

