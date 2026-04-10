use iced::{Color, Font, font};

// ── Figma palette ───────────────────────────────────────────────────────────

pub const BG: Color = Color::from_rgb(
    0x0D as f32 / 255.0,
    0x0C as f32 / 255.0,
    0x0B as f32 / 255.0,
);
pub const RAIL: Color = Color::from_rgb(
    0x12 as f32 / 255.0,
    0x10 as f32 / 255.0,
    0x0F as f32 / 255.0,
);
pub const SURF: Color = Color::from_rgb(
    0x18 as f32 / 255.0,
    0x16 as f32 / 255.0,
    0x14 as f32 / 255.0,
);
pub const SURF2: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x1D as f32 / 255.0,
    0x1A as f32 / 255.0,
);
pub const BORDER: Color = Color::from_rgb(
    0x2D as f32 / 255.0,
    0x28 as f32 / 255.0,
    0x24 as f32 / 255.0,
);
pub const AMBER: Color = Color::from_rgb(
    0xC1 as f32 / 255.0,
    0x7D as f32 / 255.0,
    0x3A as f32 / 255.0,
);
pub const GREEN: Color = Color::from_rgb(
    0x58 as f32 / 255.0,
    0xA6 as f32 / 255.0,
    0x6E as f32 / 255.0,
);
pub const TEXT_ON: Color = Color::from_rgb(
    0xE6 as f32 / 255.0,
    0xDC as f32 / 255.0,
    0xD2 as f32 / 255.0,
);
pub const DIM: Color = Color::from_rgb(
    0x96 as f32 / 255.0,
    0x8A as f32 / 255.0,
    0x7E as f32 / 255.0,
);
pub const MUTE: Color = Color::from_rgb(
    0x5A as f32 / 255.0,
    0x52 as f32 / 255.0,
    0x4A as f32 / 255.0,
);

// ── Typography ───────────────────────────────────────────────────────────────

pub const MONO: Font = Font {
    family: font::Family::Name("JetBrains Mono"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    style: font::Style::Normal,
};


