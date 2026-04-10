use std::collections::HashMap;

use iced::widget::{canvas, column, text, Space};
use iced::{mouse, Color, Element, Fill, Point, Rectangle, Renderer, Size, Theme};

use crate::message::Message;
use crate::theme as pal;
use crate::ui::widgets::canvas_utils::rounded_rect;
use crate::ui::widgets::color_utils::{adjust_color, blend, read_color};

// ── Public entry point ───────────────────────────────────────────────────────

pub fn kvantum_preview<'a>(
    slider_values: &'a HashMap<String, f32>,
    toggle_values: &'a HashMap<String, bool>,
    text_input_values: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let contrast = slider_values.get("kvantum.general.contrast").copied().unwrap_or(10.0) / 10.0;
    let intensity = slider_values.get("kvantum.general.intensity").copied().unwrap_or(10.0) / 10.0;
    let saturation = slider_values.get("kvantum.general.saturation").copied().unwrap_or(10.0) / 10.0;
    let c = |key: &str, def: Color| {
        adjust_color(read_color(key, text_input_values, slider_values, def), contrast, intensity, saturation)
    };
    let tgl = |key: &str, def: bool| toggle_values.get(key).copied().unwrap_or(def);
    let sl = |key: &str, def: f32| slider_values.get(key).copied().unwrap_or(def);

    let preview = WidgetPreview {
        // Colors
        window_color:        c("kvantum.color.window.color",        Color::from_rgb(0.14, 0.14, 0.14)),
        base_color:          c("kvantum.color.base.color",          Color::from_rgb(0.12, 0.12, 0.12)),
        alt_base_color:      c("kvantum.color.alt.base.color",      Color::from_rgb(0.15, 0.15, 0.15)),
        button_color:        c("kvantum.color.button.color",        Color::from_rgb(0.22, 0.22, 0.22)),
        light_color:         c("kvantum.color.light.color",         Color::from_rgb(0.35, 0.35, 0.35)),
        mid_color:           c("kvantum.color.mid.color",           Color::from_rgb(0.25, 0.25, 0.25)),
        dark_color:          c("kvantum.color.dark.color",          Color::from_rgb(0.08, 0.08, 0.08)),
        highlight_color:     c("kvantum.color.highlight.color",     Color::from_rgb(0.33, 0.65, 0.43)),
        text_color:          c("kvantum.color.text.color",          Color::from_rgb(0.9, 0.86, 0.82)),
        window_text_color:   c("kvantum.color.window.text.color",   Color::from_rgb(0.9, 0.86, 0.82)),
        button_text_color:   c("kvantum.color.button.text.color",   Color::from_rgb(0.9, 0.86, 0.82)),
        disabled_text_color: c("kvantum.color.disabled.text.color", Color::from_rgb(0.45, 0.42, 0.38)),
        highlight_text_color:c("kvantum.color.highlight.text.color",Color::WHITE),
        link_color:          c("kvantum.color.link.color",          Color::from_rgb(0.35, 0.55, 0.85)),
        link_visited_color:  c("kvantum.color.link.visited.color",  Color::from_rgb(0.55, 0.35, 0.75)),
        // General — Visual toggles
        composite:           tgl("kvantum.general.composite", true),
        translucent_windows: tgl("kvantum.general.translucent_windows", false),
        blurring:            tgl("kvantum.general.blurring", false),
        popup_blurring:      tgl("kvantum.general.popup_blurring", false),
        animate_states:      tgl("kvantum.general.animate_states", true),
        fill_rubberband:     tgl("kvantum.general.fill_rubberband", false),
        no_window_pattern:   tgl("kvantum.general.no_window_pattern", false),
        shadowless_popup:    tgl("kvantum.general.shadowless_popup", false),
        // General — Behavior toggles
        scroll_arrows:       tgl("kvantum.general.scroll_arrows", false),
        // General — Opacity
        reduce_window_opacity: (sl("kvantum.window_opacity_reduction", 0.0) / 100.0).clamp(0.0, 1.0),
        reduce_menu_opacity:   (sl("kvantum.reduce_opacity", 0.0) / 100.0).clamp(0.0, 1.0),
        // General — Sizing
        small_icon_size:     sl("kvantum.general.small_icon_size", 16.0),
        large_icon_size:     sl("kvantum.general.large_icon_size", 32.0),
        slider_width:        sl("kvantum.general.slider_width", 8.0),
        slider_handle_width: sl("kvantum.general.slider_handle_width", 20.0),
        slider_handle_length:sl("kvantum.general.slider_handle_length", 20.0),
        // General — Layout
        layout_spacing:      sl("kvantum.general.layout_spacing", 6.0),
        layout_margin:       sl("kvantum.general.layout_margin", 4.0),
        // General — Effects
        scrollbar_width:     sl("kvantum.general.scroll_width", 12.0),
        progress_thickness:  sl("kvantum.general.progressbar_thickness", 6.0),
        menu_shadow_depth:   sl("kvantum.general.menu_shadow_depth", 6.0),
        tooltip_shadow_depth:sl("kvantum.general.tooltip_shadow_depth", 6.0),
        splitter_width:      sl("kvantum.general.splitter_width", 7.0),
        arrow_size:          sl("kvantum.general.arrow_size", 9.0),
    };

    column![
        text("Preview")
            .size(16)
            .color(pal::TEXT_ON)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        Space::new().height(12),
        canvas(preview).width(Fill).height(600),
    ]
    .into()
}

// ── WidgetPreview struct ─────────────────────────────────────────────────────

struct WidgetPreview {
    // Colors
    window_color: Color,
    base_color: Color,
    alt_base_color: Color,
    button_color: Color,
    light_color: Color,
    mid_color: Color,
    dark_color: Color,
    highlight_color: Color,
    text_color: Color,
    window_text_color: Color,
    button_text_color: Color,
    disabled_text_color: Color,
    highlight_text_color: Color,
    link_color: Color,
    link_visited_color: Color,
    // General — Visual toggles
    composite: bool,
    translucent_windows: bool,
    blurring: bool,
    popup_blurring: bool,
    animate_states: bool,
    fill_rubberband: bool,
    no_window_pattern: bool,
    shadowless_popup: bool,
    // General — Behavior
    scroll_arrows: bool,
    // General — Opacity
    reduce_window_opacity: f32,
    reduce_menu_opacity: f32,
    // General — Sizing
    small_icon_size: f32,
    large_icon_size: f32,
    slider_width: f32,
    slider_handle_width: f32,
    slider_handle_length: f32,
    // General — Layout
    layout_spacing: f32,
    layout_margin: f32,
    // General — Effects
    scrollbar_width: f32,
    progress_thickness: f32,
    menu_shadow_depth: f32,
    tooltip_shadow_depth: f32,
    splitter_width: f32,
    arrow_size: f32,
}

impl<Message> canvas::Program<Message> for WidgetPreview {
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

        let m = self.layout_margin.clamp(2.0, 20.0) + 8.0;
        let w = bounds.width - 2.0 * m - self.scrollbar_width.clamp(4.0, 24.0) - 8.0;

        // Translucency: draw checkerboard first, then window_color with alpha on top
        let is_translucent = self.translucent_windows && self.reduce_window_opacity > 0.01;
        if is_translucent {
            // Checkerboard pattern (like Photoshop transparency indicator)
            let check = 8.0;
            let c1 = Color::from_rgb(0.25, 0.25, 0.25);
            let c2 = Color::from_rgb(0.18, 0.18, 0.18);
            let cols = (bounds.width / check).ceil() as i32;
            let rows = (bounds.height / check).ceil() as i32;
            for ry in 0..rows {
                for cx in 0..cols {
                    let color = if (ry + cx) % 2 == 0 { c1 } else { c2 };
                    frame.fill(
                        &canvas::Path::rectangle(
                            Point::new(cx as f32 * check, ry as f32 * check),
                            Size::new(check, check),
                        ),
                        color,
                    );
                }
            }
        }

        // Window background with opacity
        let bg_alpha = if is_translucent {
            (1.0 - self.reduce_window_opacity).clamp(0.1, 1.0)
        } else {
            1.0
        };
        frame.fill(
            &canvas::Path::rectangle(Point::ORIGIN, bounds.size()),
            Color { a: bg_alpha, ..self.window_color },
        );

        // Blurring indicator (frosted glass effect)
        if self.blurring && is_translucent {
            // Simulate blur with semi-transparent overlay
            frame.fill(
                &canvas::Path::rectangle(Point::ORIGIN, bounds.size()),
                Color { a: 0.15, ..self.window_color },
            );
        }

        let mut y = m;
        let gap = self.layout_spacing.clamp(2.0, 16.0);

        // — Push Buttons —
        y = section_label(&mut frame, m, y, w, "Push Buttons", self.window_text_color);
        y = draw_buttons(&mut frame, m, y, w, self);

        // — Text Input —
        y = section_label(&mut frame, m, y + gap, w, "Text Input", self.window_text_color);
        y = draw_text_input(&mut frame, m, y + gap, w, self);

        // — Checkboxes & Radio —
        y = section_label(&mut frame, m, y + gap, w, "Checkboxes & Radio", self.window_text_color);
        y = draw_checkboxes(&mut frame, m, y + gap, self);
        y = draw_radio_buttons(&mut frame, m, y + gap, self);

        // — List Selection —
        y = section_label(&mut frame, m, y + gap, w, "List Selection", self.window_text_color);
        y = draw_list_items(&mut frame, m, y + gap, w, self);

        // — Tabs —
        y = section_label(&mut frame, m, y + gap, w, "Tabs", self.window_text_color);
        y = draw_tabs(&mut frame, m, y + gap, w, self);

        // — Slider —
        y = section_label(&mut frame, m, y + gap, w, "Slider", self.window_text_color);
        y = draw_slider(&mut frame, m, y + gap, w, self);

        // — Progress Bar —
        y = section_label(&mut frame, m, y + gap, w, "Progress Bar", self.window_text_color);
        y = draw_progress_bar(&mut frame, m, y + gap, w, self);

        // — Combo with arrow —
        y = section_label(&mut frame, m, y + gap, w, "ComboBox", self.window_text_color);
        y = draw_combobox(&mut frame, m, y + gap, w, self);

        // — Tooltip —
        y = section_label(&mut frame, m, y + gap, w, "Tooltip", self.window_text_color);
        y = draw_tooltip(&mut frame, m, y + gap, self);

        // — Splitter —
        y = section_label(&mut frame, m, y + gap, w, "Splitter", self.window_text_color);
        y = draw_splitter(&mut frame, m, y + gap, w, self);

        // — Icons —
        y = section_label(&mut frame, m, y + gap, w, "Icons", self.window_text_color);
        y = draw_icons(&mut frame, m, y + gap, self);

        // — Disabled & Links —
        y = section_label(&mut frame, m, y + gap, w, "Other", self.window_text_color);
        y = draw_misc_text(&mut frame, m, y + gap, self);

        // — General settings status —
        draw_general_status(&mut frame, m, y + gap, self);

        // — Scrollbar —
        draw_scrollbar(&mut frame, bounds.width, bounds.height, self);

        vec![frame.into_geometry()]
    }
}

// ── Drawing functions ────────────────────────────────────────────────────────

fn section_label(
    frame: &mut canvas::Frame,
    x: f32, y: f32, _w: f32,
    label: &str,
    color: Color,
) -> f32 {
    frame.fill_text(canvas::Text {
        content: label.to_string(),
        position: Point::new(x, y),
        color: Color { a: 0.5, ..color },
        size: iced::Pixels(11.0),
        ..Default::default()
    });
    y + 16.0
}

fn draw_buttons(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let btn_w = (w - 16.0) / 3.0;
    let btn_h = 30.0;
    let labels = ["Normal", "Hovered", "Pressed"];

    for (i, label) in labels.iter().enumerate() {
        let bx = x + i as f32 * (btn_w + 8.0);

        let (bg, text_c) = match i {
            1 => (blend(p.button_color, p.light_color, 0.3), p.button_text_color),
            2 => (p.highlight_color, p.highlight_text_color),
            _ => (p.button_color, p.button_text_color),
        };

        // 3D effect: light top/left, dark bottom/right
        frame.fill(&rounded_rect(bx, y, btn_w, btn_h, 4.0), bg);
        frame.stroke(
            &canvas::Path::line(Point::new(bx + 1.0, y + 0.5), Point::new(bx + btn_w - 1.0, y + 0.5)),
            canvas::Stroke::default().with_color(if i == 2 { p.dark_color } else { p.light_color }).with_width(1.0),
        );
        frame.stroke(
            &canvas::Path::line(Point::new(bx + 0.5, y + 1.0), Point::new(bx + 0.5, y + btn_h - 1.0)),
            canvas::Stroke::default().with_color(if i == 2 { p.dark_color } else { p.light_color }).with_width(1.0),
        );
        frame.stroke(
            &canvas::Path::line(Point::new(bx + 1.0, y + btn_h - 0.5), Point::new(bx + btn_w - 1.0, y + btn_h - 0.5)),
            canvas::Stroke::default().with_color(if i == 2 { p.light_color } else { p.dark_color }).with_width(1.0),
        );
        frame.stroke(
            &canvas::Path::line(Point::new(bx + btn_w - 0.5, y + 1.0), Point::new(bx + btn_w - 0.5, y + btn_h - 1.0)),
            canvas::Stroke::default().with_color(if i == 2 { p.light_color } else { p.dark_color }).with_width(1.0),
        );

        frame.fill_text(canvas::Text {
            content: label.to_string(),
            position: Point::new(bx + btn_w / 2.0, y + btn_h / 2.0),
            color: text_c,
            size: iced::Pixels(12.0),
            align_x: iced::alignment::Horizontal::Center.into(),
            align_y: iced::alignment::Vertical::Center,
            ..Default::default()
        });
    }

    y + btn_h
}

fn draw_text_input(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let h = 28.0;
    frame.fill(&rounded_rect(x, y, w, h, 3.0), p.base_color);
    frame.stroke(
        &rounded_rect(x, y, w, h, 3.0),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill_text(canvas::Text {
        content: "Sample input text".to_string(),
        position: Point::new(x + 8.0, y + h / 2.0),
        color: p.text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });
    // Cursor
    frame.stroke(
        &canvas::Path::line(Point::new(x + 108.0, y + 5.0), Point::new(x + 108.0, y + h - 5.0)),
        canvas::Stroke::default().with_color(p.text_color).with_width(1.0),
    );
    y + h
}

fn draw_checkboxes(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) -> f32 {
    let size = 14.0;
    // Checked
    frame.fill(&canvas::Path::rectangle(Point::new(x, y), Size::new(size, size)), p.base_color);
    frame.stroke(
        &canvas::Path::rectangle(Point::new(x, y), Size::new(size, size)),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    // Checkmark
    let check = canvas::Path::new(|b| {
        b.move_to(Point::new(x + 3.0, y + 7.0));
        b.line_to(Point::new(x + 6.0, y + 11.0));
        b.line_to(Point::new(x + 11.0, y + 3.0));
    });
    frame.stroke(&check, canvas::Stroke::default().with_color(p.highlight_color).with_width(2.0));
    frame.fill_text(canvas::Text {
        content: "Checked".to_string(),
        position: Point::new(x + size + 6.0, y + size / 2.0),
        color: p.window_text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    // Unchecked
    let ux = x + 120.0;
    frame.fill(&canvas::Path::rectangle(Point::new(ux, y), Size::new(size, size)), p.base_color);
    frame.stroke(
        &canvas::Path::rectangle(Point::new(ux, y), Size::new(size, size)),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill_text(canvas::Text {
        content: "Unchecked".to_string(),
        position: Point::new(ux + size + 6.0, y + size / 2.0),
        color: p.window_text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + size + 4.0
}

fn draw_radio_buttons(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) -> f32 {
    let r = 7.0;
    // Selected
    frame.stroke(
        &canvas::Path::circle(Point::new(x + r, y + r), r),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill(&canvas::Path::circle(Point::new(x + r, y + r), r - 1.0), p.base_color);
    frame.fill(&canvas::Path::circle(Point::new(x + r, y + r), 4.0), p.highlight_color);
    frame.fill_text(canvas::Text {
        content: "Selected".to_string(),
        position: Point::new(x + r * 2.0 + 6.0, y + r),
        color: p.window_text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    // Unselected
    let ux = x + 120.0;
    frame.stroke(
        &canvas::Path::circle(Point::new(ux + r, y + r), r),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill(&canvas::Path::circle(Point::new(ux + r, y + r), r - 1.0), p.base_color);
    frame.fill_text(canvas::Text {
        content: "Unselected".to_string(),
        position: Point::new(ux + r * 2.0 + 6.0, y + r),
        color: p.window_text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + r * 2.0 + 4.0
}

fn draw_list_items(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let row_h = 24.0;
    // Apply menu opacity reduction to list items
    let menu_alpha = (1.0 - p.reduce_menu_opacity).clamp(0.3, 1.0);
    let items = [
        ("Normal item", Color { a: menu_alpha, ..p.base_color }, p.text_color),
        ("Selected item", Color { a: menu_alpha, ..p.highlight_color }, p.highlight_text_color),
        ("Alternate item", Color { a: menu_alpha, ..p.alt_base_color }, p.text_color),
    ];

    for (i, (label, bg, fg)) in items.iter().enumerate() {
        let ry = y + i as f32 * row_h;
        frame.fill(&canvas::Path::rectangle(Point::new(x, ry), Size::new(w, row_h)), *bg);
        frame.fill_text(canvas::Text {
            content: label.to_string(),
            position: Point::new(x + 8.0, ry + row_h / 2.0),
            color: *fg,
            size: iced::Pixels(12.0),
            align_y: iced::alignment::Vertical::Center,
            ..Default::default()
        });
    }

    // Border
    frame.stroke(
        &canvas::Path::rectangle(Point::new(x, y), Size::new(w, row_h * 3.0)),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );

    // Shadow (uses menu_shadow_depth)
    let shadow_d = p.menu_shadow_depth.clamp(0.0, 20.0);
    if shadow_d > 0.0 {
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(x + 2.0, y + row_h * 3.0),
                Size::new(w, shadow_d.min(4.0)),
            ),
            Color { a: 0.15, ..p.dark_color },
        );
    }

    y + row_h * 3.0 + shadow_d.min(4.0)
}

fn draw_tabs(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let tab_h = 28.0;
    let tab_w = w / 3.0;
    let labels = ["Tab 1", "Tab 2", "Tab 3"];

    for (i, label) in labels.iter().enumerate() {
        let tx = x + i as f32 * tab_w;
        let is_active = i == 0;
        let bg = if is_active { p.window_color } else { p.button_color };
        let fg = if is_active { p.window_text_color } else { blend(p.window_text_color, p.mid_color, 0.4) };

        frame.fill(&canvas::Path::rectangle(Point::new(tx, y), Size::new(tab_w, tab_h)), bg);
        frame.fill_text(canvas::Text {
            content: label.to_string(),
            position: Point::new(tx + tab_w / 2.0, y + tab_h / 2.0),
            color: fg,
            size: iced::Pixels(12.0),
            align_x: iced::alignment::Horizontal::Center.into(),
            align_y: iced::alignment::Vertical::Center,
            ..Default::default()
        });

        if is_active {
            frame.fill(
                &canvas::Path::rectangle(Point::new(tx, y + tab_h - 2.0), Size::new(tab_w, 2.0)),
                p.highlight_color,
            );
        }
    }

    // Bottom line
    frame.stroke(
        &canvas::Path::line(Point::new(x, y + tab_h), Point::new(x + w, y + tab_h)),
        canvas::Stroke::default().with_color(p.dark_color).with_width(1.0),
    );

    y + tab_h
}

fn draw_slider(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let track_h = p.slider_width.clamp(2.0, 20.0);
    let handle_w = p.slider_handle_width.clamp(8.0, 40.0);
    let handle_h = p.slider_handle_length.clamp(8.0, 40.0).max(track_h + 4.0);
    let cy = y + handle_h / 2.0;

    // Track
    frame.fill(
        &rounded_rect(x, cy - track_h / 2.0, w, track_h, track_h / 2.0),
        p.mid_color,
    );
    // Filled portion (40%)
    let fill_w = w * 0.4;
    frame.fill(
        &rounded_rect(x, cy - track_h / 2.0, fill_w, track_h, track_h / 2.0),
        p.highlight_color,
    );
    // Handle
    let hx = x + fill_w - handle_w / 2.0;
    frame.fill(
        &rounded_rect(hx, cy - handle_h / 2.0, handle_w, handle_h, 4.0),
        p.button_color,
    );
    frame.stroke(
        &rounded_rect(hx, cy - handle_h / 2.0, handle_w, handle_h, 4.0),
        canvas::Stroke::default().with_color(p.light_color).with_width(1.0),
    );

    y + handle_h
}

fn draw_progress_bar(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let h = p.progress_thickness.clamp(4.0, 30.0);
    let fill_pct = 0.6;

    // Background
    frame.fill(&rounded_rect(x, y, w, h, 3.0), p.mid_color);
    // Fill
    let fill_w = w * fill_pct;
    frame.fill(&rounded_rect(x, y, fill_w, h, 3.0), p.highlight_color);
    // Text
    frame.fill_text(canvas::Text {
        content: "60%".to_string(),
        position: Point::new(x + w / 2.0, y + h / 2.0),
        color: p.highlight_text_color,
        size: iced::Pixels(11.0),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + h
}

fn draw_misc_text(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) -> f32 {
    frame.fill_text(canvas::Text {
        content: "Disabled text".to_string(),
        position: Point::new(x, y),
        color: p.disabled_text_color,
        size: iced::Pixels(12.0),
        ..Default::default()
    });
    frame.fill_text(canvas::Text {
        content: "Link".to_string(),
        position: Point::new(x + 120.0, y),
        color: p.link_color,
        size: iced::Pixels(12.0),
        ..Default::default()
    });
    frame.fill_text(canvas::Text {
        content: "Visited link".to_string(),
        position: Point::new(x + 170.0, y),
        color: p.link_visited_color,
        size: iced::Pixels(12.0),
        ..Default::default()
    });
    y + 16.0
}

fn draw_combobox(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let h = 28.0;
    let combo_w = w.min(200.0);
    let arrow_w = p.arrow_size.clamp(6.0, 24.0) + 8.0;

    // Main field
    frame.fill(&rounded_rect(x, y, combo_w, h, 3.0), p.base_color);
    frame.stroke(&rounded_rect(x, y, combo_w, h, 3.0),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0));
    frame.fill_text(canvas::Text {
        content: "Selected option".to_string(),
        position: Point::new(x + 8.0, y + h / 2.0),
        color: p.text_color,
        size: iced::Pixels(12.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    // Arrow button
    let arr_x = x + combo_w - arrow_w;
    frame.fill(&canvas::Path::rectangle(Point::new(arr_x, y), Size::new(arrow_w, h)), p.button_color);
    let arr_s = p.arrow_size.clamp(6.0, 24.0) * 0.35;
    let cx = arr_x + arrow_w / 2.0;
    let cy = y + h / 2.0;
    let arrow = canvas::Path::new(|b| {
        b.move_to(Point::new(cx, cy + arr_s));
        b.line_to(Point::new(cx - arr_s, cy - arr_s));
        b.line_to(Point::new(cx + arr_s, cy - arr_s));
        b.close();
    });
    frame.fill(&arrow, p.button_text_color);

    y + h
}

fn draw_tooltip(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) -> f32 {
    let tip_w = 140.0;
    let tip_h = 24.0;
    let shadow_d = p.tooltip_shadow_depth.clamp(0.0, 16.0);

    // Shadow (if not shadowless)
    if !p.shadowless_popup && shadow_d > 0.0 {
        frame.fill(
            &rounded_rect(x + 2.0, y + 2.0, tip_w + shadow_d, tip_h + shadow_d, 3.0),
            Color { a: 0.2, ..p.dark_color },
        );
    }

    // Tooltip body
    let bg = Color::from_rgb(
        (p.base_color.r + 0.05).clamp(0.0, 1.0),
        (p.base_color.g + 0.05).clamp(0.0, 1.0),
        (p.base_color.b + 0.05).clamp(0.0, 1.0),
    );
    frame.fill(&rounded_rect(x, y, tip_w, tip_h, 3.0), bg);
    frame.stroke(&rounded_rect(x, y, tip_w, tip_h, 3.0),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0));
    frame.fill_text(canvas::Text {
        content: "Tooltip text".to_string(),
        position: Point::new(x + 8.0, y + tip_h / 2.0),
        color: p.text_color,
        size: iced::Pixels(11.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + tip_h + shadow_d.min(4.0)
}

fn draw_splitter(
    frame: &mut canvas::Frame,
    x: f32, y: f32, w: f32,
    p: &WidgetPreview,
) -> f32 {
    let sp_w = p.splitter_width.clamp(1.0, 12.0);
    let panel_h = 30.0;
    let left_w = (w - sp_w) / 2.0;
    let right_w = w - left_w - sp_w;

    // Left panel
    frame.fill(&canvas::Path::rectangle(Point::new(x, y), Size::new(left_w, panel_h)), p.base_color);
    frame.fill_text(canvas::Text {
        content: "Left".to_string(),
        position: Point::new(x + left_w / 2.0, y + panel_h / 2.0),
        color: Color { a: 0.5, ..p.text_color },
        size: iced::Pixels(11.0),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    // Splitter
    frame.fill(
        &canvas::Path::rectangle(Point::new(x + left_w, y), Size::new(sp_w, panel_h)),
        p.mid_color,
    );
    // Handle dots
    let cx = x + left_w + sp_w / 2.0;
    for i in 0..3 {
        frame.fill(
            &canvas::Path::circle(Point::new(cx, y + panel_h / 2.0 - 4.0 + i as f32 * 4.0), 1.5),
            p.light_color,
        );
    }

    // Right panel
    frame.fill(&canvas::Path::rectangle(Point::new(x + left_w + sp_w, y), Size::new(right_w, panel_h)), p.alt_base_color);
    frame.fill_text(canvas::Text {
        content: "Right".to_string(),
        position: Point::new(x + left_w + sp_w + right_w / 2.0, y + panel_h / 2.0),
        color: Color { a: 0.5, ..p.text_color },
        size: iced::Pixels(11.0),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + panel_h
}

fn draw_icons(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) -> f32 {
    let small = p.small_icon_size.clamp(8.0, 32.0);
    let large = p.large_icon_size.clamp(16.0, 64.0);

    // Small icon placeholder
    frame.stroke(
        &canvas::Path::rectangle(Point::new(x, y), Size::new(small, small)),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill_text(canvas::Text {
        content: "\u{25A3}".to_string(), // filled square icon
        position: Point::new(x + small / 2.0, y + small / 2.0),
        color: p.highlight_color,
        size: iced::Pixels(small * 0.6),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });
    frame.fill_text(canvas::Text {
        content: format!("{}px", small as i32),
        position: Point::new(x + small + 6.0, y + small / 2.0),
        color: Color { a: 0.5, ..p.window_text_color },
        size: iced::Pixels(10.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    // Large icon placeholder
    let lx = x + 80.0;
    frame.stroke(
        &canvas::Path::rectangle(Point::new(lx, y), Size::new(large, large)),
        canvas::Stroke::default().with_color(p.mid_color).with_width(1.0),
    );
    frame.fill_text(canvas::Text {
        content: "\u{25A3}".to_string(),
        position: Point::new(lx + large / 2.0, y + large / 2.0),
        color: p.highlight_color,
        size: iced::Pixels(large * 0.5),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });
    frame.fill_text(canvas::Text {
        content: format!("{}px", large as i32),
        position: Point::new(lx + large + 6.0, y + large / 2.0),
        color: Color { a: 0.5, ..p.window_text_color },
        size: iced::Pixels(10.0),
        align_y: iced::alignment::Vertical::Center,
        ..Default::default()
    });

    y + large.max(small)
}

fn draw_general_status(
    frame: &mut canvas::Frame,
    x: f32, y: f32,
    p: &WidgetPreview,
) {
    let mut active = Vec::new();
    if p.composite { active.push("Composite"); }
    if p.translucent_windows { active.push("Translucent"); }
    if p.blurring { active.push("Blur"); }
    if p.popup_blurring { active.push("Popup blur"); }
    if p.animate_states { active.push("Animated"); }
    if p.fill_rubberband { active.push("Fill rubber"); }
    if p.no_window_pattern { active.push("No pattern"); }
    if p.shadowless_popup { active.push("No popup shadow"); }
    if p.scroll_arrows { active.push("Scroll arrows"); }

    if !active.is_empty() {
        frame.fill_text(canvas::Text {
            content: format!("Active: {}", active.join(" · ")),
            position: Point::new(x, y),
            color: Color { a: 0.4, ..p.window_text_color },
            size: iced::Pixels(10.0),
            ..Default::default()
        });
    }
}

fn draw_scrollbar(
    frame: &mut canvas::Frame,
    canvas_w: f32, canvas_h: f32,
    p: &WidgetPreview,
) {
    let track_w = p.scrollbar_width.clamp(4.0, 24.0);
    let track_x = canvas_w - track_w - 4.0;
    let arrow_h = if p.scroll_arrows { p.arrow_size.clamp(6.0, 24.0) } else { 0.0 };
    let track_y = 16.0 + arrow_h;
    let track_h = canvas_h - 32.0 - arrow_h * 2.0;

    // Scroll arrows (if enabled)
    if p.scroll_arrows {
        let arr_s = (track_w * 0.35).min(arrow_h * 0.4);
        let cx = track_x + track_w / 2.0;

        // Up arrow
        frame.fill(&canvas::Path::rectangle(Point::new(track_x, 16.0), Size::new(track_w, arrow_h)), p.button_color);
        let up = canvas::Path::new(|b| {
            b.move_to(Point::new(cx, 16.0 + arrow_h * 0.3));
            b.line_to(Point::new(cx - arr_s, 16.0 + arrow_h * 0.7));
            b.line_to(Point::new(cx + arr_s, 16.0 + arrow_h * 0.7));
            b.close();
        });
        frame.fill(&up, p.button_text_color);

        // Down arrow
        let dy = canvas_h - 16.0 - arrow_h;
        frame.fill(&canvas::Path::rectangle(Point::new(track_x, dy), Size::new(track_w, arrow_h)), p.button_color);
        let down = canvas::Path::new(|b| {
            b.move_to(Point::new(cx, dy + arrow_h * 0.7));
            b.line_to(Point::new(cx - arr_s, dy + arrow_h * 0.3));
            b.line_to(Point::new(cx + arr_s, dy + arrow_h * 0.3));
            b.close();
        });
        frame.fill(&down, p.button_text_color);
    }

    // Track
    frame.fill(
        &rounded_rect(track_x, track_y, track_w, track_h, track_w / 2.0),
        p.base_color,
    );
    // Thumb (~30% height, at 20% from top)
    let thumb_h = track_h * 0.3;
    let thumb_y = track_y + track_h * 0.2;
    frame.fill(
        &rounded_rect(track_x, thumb_y, track_w, thumb_h, track_w / 2.0),
        p.button_color,
    );
}
