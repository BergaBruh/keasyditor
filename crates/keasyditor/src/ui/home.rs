use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{KvantumMessage, Message, Page, SettingsMessage};
use crate::theme;
use crate::ui::widgets::color_utils::{parse_hex, luminance};

/// Home page: header, two editor cards, wallpaper colors, installed themes.
#[allow(clippy::too_many_arguments)]
pub fn home_page<'a>(
    installed_themes: &'a [(String, bool)],
    matugen_loading: bool,
    matugen_palette: &'a Option<Vec<(String, String)>>,
    prefer_dark: bool,
    klassy_installed: bool,
    kvantum_installed: bool,
    matugen_installed: bool,
    active_kvantum_theme: Option<&'a str>,
) -> Element<'a, Message> {
    let header = column![
        text(t("home.title"))
            .size(32)
            .color(theme::AMBER)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
        text(t("home.subtitle"))
            .size(16)
            .color(theme::DIM),
    ]
    .spacing(4);

    // Two editor cards side by side (or "not installed" notices)
    let klassy_card: Element<'_, Message> = if klassy_installed {
        editor_card_with_extra(
            t("home.klassy_card.title"),
            t("home.klassy_card.description"),
            theme::AMBER,
            Message::NavigateTo(Page::Klassy),
            None,
        )
    } else {
        not_installed_card("Klassy", t("home.klassy_card.not_installed"), theme::AMBER)
    };

    let kvantum_card: Element<'_, Message> = if kvantum_installed {
        editor_card_with_extra(
            t("home.kvantum_card.title"),
            t("home.kvantum_card.description"),
            theme::GREEN,
            Message::NavigateTo(Page::Kvantum),
            Some((t("home.kvantum_card.new_theme"), Message::Kvantum(KvantumMessage::NewTheme))),
        )
    } else {
        not_installed_card("Kvantum", t("home.kvantum_card.not_installed"), theme::GREEN)
    };

    let cards = row![klassy_card, kvantum_card].spacing(24);

    // Wallpaper Colors section with dark/light toggle
    let palette_title = text(t("home.wallpaper_colors"))
        .size(20)
        .color(theme::TEXT_ON)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        });

    let dark_btn = theme_mode_button(t("home.palette.dark"), prefer_dark);
    let light_btn = theme_mode_button(t("home.palette.light"), !prefer_dark);
    let mode_toggle = row![dark_btn, light_btn].spacing(4);

    let palette_header = row![palette_title, Space::new().width(Fill), mode_toggle]
        .align_y(iced::Alignment::Center);

    let palette_section = build_palette_section(matugen_loading, matugen_palette, matugen_installed);

    // Installed themes section
    let themes_title = text(t("home.themes.installed"))
        .size(20)
        .color(theme::TEXT_ON)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        });

    let themes_section: Element<'a, Message> = if installed_themes.is_empty() {
        container(
            text(t("home.themes.none_found"))
                .size(13)
                .color(theme::MUTE),
        )
        .width(Fill)
        .padding([24, 24])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into()
    } else {
        // Separate user and system themes
        let user_themes: Vec<&(String, bool)> =
            installed_themes.iter().filter(|(_, sys)| !sys).collect();
        let system_themes: Vec<&(String, bool)> =
            installed_themes.iter().filter(|(_, sys)| *sys).collect();

        let mut themes_col = column![].spacing(12);

        if !user_themes.is_empty() {
            themes_col = themes_col.push(
                text(t("home.themes.user"))
                    .size(11)
                    .color(theme::MUTE),
            );
            themes_col = themes_col.push(wrap_chips(
                &user_themes.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
                false,
                active_kvantum_theme,
            ));
        }

        if !system_themes.is_empty() {
            themes_col = themes_col.push(Space::new().height(4));
            themes_col = themes_col.push(
                text(t("home.themes.system"))
                    .size(11)
                    .color(theme::MUTE),
            );
            themes_col = themes_col.push(wrap_chips(
                &system_themes.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
                true,
                active_kvantum_theme,
            ));
        }

        themes_col.into()
    };

    let content = column![
        header,
        Space::new().height(32),
        cards,
        Space::new().height(40),
        palette_header,
        Space::new().height(12),
        palette_section,
        Space::new().height(40),
        themes_title,
        Space::new().height(12),
        themes_section,
    ];

    scrollable(container(content).padding(32).width(Fill)).into()
}

/// Build the wallpaper palette display (auto-extracted on startup).
/// Matches the Flutter implementation: colored 100px tiles in a Wrap layout.
fn build_palette_section<'a>(
    loading: bool,
    palette: &'a Option<Vec<(String, String)>>,
    matugen_installed: bool,
) -> Element<'a, Message> {
    if !matugen_installed {
        return container(
            column![
                text(t("home.matugen.not_installed"))
                    .size(14)
                    .color(theme::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                Space::new().height(4),
                text(t("home.matugen.description"))
                    .size(13)
                    .color(theme::MUTE),
            ]
            .spacing(0),
        )
        .width(Fill)
        .padding([20, 24])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into();
    }

    if loading {
        return container(
            text(t("home.matugen.extracting"))
                .size(13)
                .color(theme::AMBER),
        )
        .width(Fill)
        .padding([24, 24])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into();
    }

    match palette {
        Some(colors) if !colors.is_empty() => {
            let mut wrap_row = row![].spacing(8);
            for (name, hex) in colors.iter() {
                wrap_row = wrap_row.push(color_swatch(name, hex));
            }

            container(wrap_row.wrap().vertical_spacing(8))
                .width(Fill)
                .padding(16)
                .style(|_: &Theme| container::Style {
                    background: Some(Background::Color(theme::SURF)),
                    border: Border {
                        color: theme::BORDER,
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                })
                .into()
        }
        _ => {
            container(
                text(t("home.matugen.no_palette"))
                    .size(13)
                    .color(theme::MUTE),
            )
            .width(Fill)
            .padding([24, 24])
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(theme::SURF)),
                border: Border {
                    color: theme::BORDER,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            })
            .into()
        }
    }
}

/// A single color swatch tile: 100px wide, colored background, name + hex overlay.
/// Mirrors the Flutter `_ColorSwatch` widget.
fn color_swatch<'a>(name: &str, hex: &str) -> Element<'a, Message> {
    let color = parse_hex(hex);
    // Determine if the color is dark (luminance < 0.4) → use light text, else dark text
    let is_dark = luminance(color) < 0.4;

    let name_color = if is_dark { theme::MUTE } else { Color::from_rgb(0.2, 0.2, 0.2) };
    let hex_color = if is_dark { theme::DIM } else { Color::from_rgb(0.33, 0.33, 0.33) };

    let display_name = name.replace('_', " ");

    let content = column![
        text(display_name)
            .size(10)
            .color(name_color)
            .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
        Space::new().height(2),
        text(hex.to_string())
            .size(10)
            .color(hex_color)
            .font(iced::Font { family: iced::font::Family::Monospace, ..Default::default() }),
    ];

    container(content)
        .width(100)
        .padding(8)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(color)),
            border: Border {
                color: theme::BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// A dark/light mode toggle button.
fn theme_mode_button(label: String, is_active: bool) -> Element<'static, Message> {
    let prefer_dark = label == t("home.palette.dark");
    let bg = if is_active { theme::SURF2 } else { theme::SURF };
    let text_color = if is_active { theme::TEXT_ON } else { theme::MUTE };
    let border_color = if is_active { theme::AMBER } else { theme::BORDER };

    button(text(label).size(12).color(text_color))
        .on_press(Message::Settings(SettingsMessage::ToggleDarkPalette(prefer_dark)))
        .padding([5, 14])
        .style(move |_: &Theme, _| button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Lay out theme chips with automatic wrapping.
fn wrap_chips<'a>(names: &[&str], is_system: bool, active: Option<&str>) -> Element<'a, Message> {
    let mut wrap_row = row![].spacing(8);
    for name in names {
        let is_active = active.is_some_and(|a| a == *name);
        wrap_row = wrap_row.push(theme_chip(name, is_system, is_active));
    }
    wrap_row.wrap().vertical_spacing(8).into()
}

/// A card shown when a theme engine is not installed.
fn not_installed_card(
    name: &'static str,
    hint: String,
    accent: Color,
) -> Element<'static, Message> {
    let content = column![
        text("\u{26A0}").size(36).color(theme::MUTE),
        Space::new().height(12),
        text(format!("{} not installed", name))
            .size(16)
            .color(theme::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        Space::new().height(8),
        text(hint).size(13).color(theme::MUTE),
    ]
    .padding(24);

    container(content)
        .width(Fill)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: Color { a: 0.15, ..accent },
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// A large card that acts as entry point to an editor.
/// `extra_button` is an optional (label, message) for an additional action button.
fn editor_card_with_extra(
    title: String,
    description: String,
    accent: Color,
    on_press: Message,
    extra_button: Option<(String, Message)>,
) -> Element<'static, Message> {
    let icon_text = if accent.r > 0.6 { "\u{25D0}" } else { "\u{25CF}" };

    let open_btn = button(
        text(t("home.open_editor")).size(13).color(theme::BG),
    )
    .on_press(on_press)
    .padding([8, 20])
    .style(move |_: &Theme, _| button::Style {
        background: Some(Background::Color(accent)),
        border: Border { radius: 6.0.into(), ..Default::default() },
        text_color: theme::BG,
        ..Default::default()
    });

    let mut btn_row = row![open_btn].spacing(8);

    if let Some((label, msg)) = extra_button {
        let extra_btn = button(
            text(label).size(13).color(accent),
        )
        .on_press(msg)
        .padding([8, 20])
        .style(move |_: &Theme, _| button::Style {
            background: None,
            text_color: accent,
            border: Border {
                color: Color { a: 0.4, ..accent },
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        });
        btn_row = btn_row.push(extra_btn);
    }

    let card_content = column![
        text(icon_text).size(36).color(accent),
        Space::new().height(12),
        text(title)
            .size(16)
            .color(theme::TEXT_ON)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
        Space::new().height(8),
        text(description).size(13).color(theme::DIM),
        Space::new().height(16),
        btn_row,
    ]
    .padding(24);

    container(card_content)
        .width(Fill)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(theme::SURF)),
            border: Border {
                color: Color { a: 0.25, ..accent },
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// A small theme chip showing a Kvantum theme name.
/// Clicking it applies that theme system-wide via kvantummanager.
/// Active theme is highlighted with an accent border.
fn theme_chip<'a>(name: &str, is_system: bool, is_active: bool) -> Element<'a, Message> {
    let dot_color = if is_active {
        theme::GREEN
    } else if is_system {
        theme::DIM
    } else {
        theme::GREEN
    };
    let name_owned = name.to_string();
    let text_color = if is_active { theme::TEXT_ON } else { theme::DIM };
    let border_color = if is_active { theme::GREEN } else { theme::BORDER };
    let bg = if is_active { theme::SURF2 } else { theme::SURF };

    let content = row![
        text("\u{25CF}").size(10).color(dot_color),
        text(name_owned.clone())
            .size(13)
            .color(text_color),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    button(content)
        .on_press(Message::Kvantum(KvantumMessage::ApplySystemTheme(
            name_owned,
        )))
        .padding([8, 16])
        .style(move |_: &Theme, _| button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                color: border_color,
                width: if is_active { 1.5 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}
