//! Shared canvas path helpers used across preview panels.

use iced::widget::canvas;
use iced::Point;

/// Build a rounded rectangle path using arc_to.
pub fn rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> canvas::Path {
    canvas::Path::new(|b| {
        let r = r.min(w / 2.0).min(h / 2.0);
        b.move_to(Point::new(x + r, y));
        b.line_to(Point::new(x + w - r, y));
        b.arc_to(Point::new(x + w, y), Point::new(x + w, y + r), r);
        b.line_to(Point::new(x + w, y + h - r));
        b.arc_to(Point::new(x + w, y + h), Point::new(x + w - r, y + h), r);
        b.line_to(Point::new(x + r, y + h));
        b.arc_to(Point::new(x, y + h), Point::new(x, y + h - r), r);
        b.line_to(Point::new(x, y + r));
        b.arc_to(Point::new(x, y), Point::new(x + r, y), r);
        b.close();
    })
}

/// Build a rectangle with only top corners rounded (for titlebars, tab headers).
pub fn rounded_rect_top(x: f32, y: f32, w: f32, h: f32, r: f32) -> canvas::Path {
    canvas::Path::new(|b| {
        let r = r.min(w / 2.0).min(h / 2.0);
        b.move_to(Point::new(x + r, y));
        b.line_to(Point::new(x + w - r, y));
        b.arc_to(Point::new(x + w, y), Point::new(x + w, y + r), r);
        b.line_to(Point::new(x + w, y + h));
        b.line_to(Point::new(x, y + h));
        b.line_to(Point::new(x, y + r));
        b.arc_to(Point::new(x, y), Point::new(x + r, y), r);
        b.close();
    })
}
