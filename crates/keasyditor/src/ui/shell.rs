use iced::widget::{button, column, container, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Fill, Theme};

use crate::i18n::t;
use crate::message::{Message, Page};
use crate::theme;

/// Builds the 72px-wide navigation rail.
pub fn nav_rail(current_page: Page, klassy_installed: bool, kvantum_installed: bool) -> Element<'static, Message> {
    // Top logo: amber rounded square with bold "K"
    let logo = container(
        text("K")
            .size(20)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(theme::BG),
    )
    .width(36)
    .height(36)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(theme::AMBER)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let logo_row = container(logo)
        .width(72)
        .padding(iced::Padding { top: 20.0, right: 18.0, bottom: 10.0, left: 18.0 })
        .align_x(iced::alignment::Horizontal::Center);

    // Divider (simulated with a thin container)
    let divider = container(
        container(Space::new().width(Fill).height(1)).style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::BORDER)),
            ..Default::default()
        }),
    )
    .padding([0, 20]);

    struct NavItem {
        label: String,
        icon: &'static str,
        page: Page,
        accent: Color,
    }

    let mut items = vec![
        NavItem {
            label: t("nav.home"),
            icon: "\u{2302}",
            page: Page::Home,
            accent: theme::AMBER,
        },
    ];

    if klassy_installed {
        items.push(NavItem {
            label: t("nav.klassy"),
            icon: "\u{25D0}",
            page: Page::Klassy,
            accent: theme::AMBER,
        });
    }

    if kvantum_installed {
        items.push(NavItem {
            label: t("nav.kvantum"),
            icon: "\u{25CF}",
            page: Page::Kvantum,
            accent: theme::GREEN,
        });
    }

    items.push(NavItem {
        label: t("nav.settings"),
        icon: "\u{2699}",
        page: Page::Settings,
        accent: theme::AMBER,
    });

    let mut nav_col = column![logo_row, divider,].spacing(0);

    for item in items {
        nav_col = nav_col.push(nav_item(
            item.icon,
            item.label,
            item.page,
            current_page,
            item.accent,
        ));
    }

    nav_col = nav_col.push(Space::new().height(Fill));

    container(nav_col)
        .width(72)
        .height(Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::RAIL)),
            ..Default::default()
        })
        .into()
}

/// A single nav rail item: 60px tall, icon + label stacked vertically.
fn nav_item(
    icon: &'static str,
    label: String,
    target: Page,
    current: Page,
    accent: Color,
) -> Element<'static, Message> {
    let is_active = current == target;

    let icon_color = if is_active { accent } else { theme::MUTE };
    let label_color = if is_active { accent } else { theme::MUTE };

    let content: Element<'static, Message> = column![
        text(icon).size(18).color(icon_color),
        text(label).size(9).color(label_color),
    ]
    .spacing(2)
    .align_x(Alignment::Center)
    .into();

    button(
        container(content)
            .width(Fill)
            .align_x(iced::alignment::Horizontal::Center),
    )
    .on_press(Message::NavigateTo(target))
    .padding([8, 0])
    .width(72)
    .height(60)
    .style(move |_: &Theme, _| button::Style {
        background: if is_active {
            Some(Background::Color(Color {
                a: 0.08,
                ..accent
            }))
        } else {
            None
        },
        text_color: icon_color,
        ..Default::default()
    })
    .into()
}
