use glory::prelude::*;
use glory::style::FlexWrap;

fn column() -> glory::ui::View {
    col().style(
        Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .gap(16.0),
    )
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let clicks = scope.cage(0usize);
        let variant = scope.cage(Some("filled".to_owned()));
        let size = scope.cage(Some("medium".to_owned()));
        let disabled = scope.cage(false);
        let loading = scope.cage(false);
        let style_theme = theme.clone();
        let button_style = scope.bond(move || {
            let size = match size.get().as_deref() {
                Some("small") => sapid::Size::Small,
                Some("large") => sapid::Size::Large,
                _ => sapid::Size::Medium,
            };
            let control = style_theme.tokens.control(size);
            let text = style_theme
                .tokens
                .typography(sapid::TypographyRole::Control, size);
            let (background, foreground, border) = if disabled.get() || loading.get() {
                (
                    style_theme.disabled,
                    style_theme.disabled_text,
                    style_theme.border,
                )
            } else {
                match variant.get().as_deref() {
                    Some("outline") => (
                        style_theme.input,
                        style_theme.accent_text,
                        style_theme.accent,
                    ),
                    Some("light") => (
                        style_theme.accent_soft,
                        style_theme.accent_text,
                        style_theme.accent_soft,
                    ),
                    _ => (
                        style_theme.accent,
                        style_theme.on_accent,
                        style_theme.accent,
                    ),
                }
            };
            Style {
                background: Some(background),
                color: Some(foreground),
                border_color: Some(border),
                font_size: Some(text.font_size),
                line_height: Some(text.line_height),
                font_weight: text.weight,
                ..Style::new()
                    .min_height(Length::Px(control.min_height))
                    .padding(Edges {
                        top: control.padding_y,
                        bottom: control.padding_y,
                        left: control.padding_x,
                        right: control.padding_x,
                    })
                    .radius(style_theme.radius * control.radius_scale)
            }
        });
        let label = scope.bond(move || {
            if loading.get() {
                return "Loading…".to_owned();
            }
            match variant.get().as_deref() {
                Some("outline") => "Outline button",
                Some("light") => "Light button",
                _ => "Filled button",
            }
            .to_owned()
        });
        view()
            .class("site-button-playground")
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_wrap(FlexWrap::Wrap)
                    .gap(24.0),
            )
            .fill((
                column()
                    .class("site-playground-preview")
                    .style(Style {
                        grow: Some(1.0),
                        align: Some(Align::Center),
                        justify: Some(Justify::Center),
                        ..Style::new()
                            .display(Display::Flex)
                            .flex_direction(FlexDirection::Column)
                            .gap(16.0)
                            .min_width(Length::Px(180.0))
                    })
                    .fill((
                        sapid::button()
                            .theme(theme.clone())
                            .class("site-playground-button")
                            .label(label)
                            .style(button_style)
                            .enabled(disabled.map(|v| !v))
                            .loading(loading)
                            .on_press(move || clicks.revise(|n| *n += 1)),
                        sapid::text()
                            .theme(theme.clone())
                            .content(clicks.map(|n| format!("Clicked {n} times"))),
                        sapid::group().theme(theme.clone()).fill(
                            [
                                ("Light variant", sapid::ButtonVariant::Light),
                                ("Outline variant", sapid::ButtonVariant::Outline),
                                ("Subtle variant", sapid::ButtonVariant::Subtle),
                            ]
                            .into_iter()
                            .map(|(label, variant)| {
                                sapid::button()
                                    .theme(theme.clone())
                                    .variant(variant)
                                    .label(label)
                                    .enabled(disabled.map(|value| !value))
                                    .loading(loading)
                                    .on_press(move || clicks.revise(|count| *count += 1))
                            })
                            .collect::<Vec<_>>(),
                        ),
                    )),
                column()
                    .class("site-playground-controls")
                    .style(
                        Style::new()
                            .display(Display::Flex)
                            .flex_direction(FlexDirection::Column)
                            .gap(16.0)
                            .width(Length::Px(208.0))
                            .max_width(Limit::Percent(100.0)),
                    )
                    .fill((
                        sapid::field("showcase-button-variant")
                            .theme(theme.clone())
                            .label("Variant")
                            .control({
                                let theme = theme.clone();
                                move |field| {
                                    sapid::native_select("showcase-variant")
                                        .theme(theme.clone())
                                        .field(field)
                                        .option(sapid::select_option("filled", "Filled"))
                                        .option(sapid::select_option("outline", "Outline"))
                                        .option(sapid::select_option("light", "Light"))
                                        .value(variant)
                                        .on_value_change(move |next| {
                                            if next.is_some() {
                                                variant.revise(|v| *v = next);
                                            }
                                        })
                                }
                            }),
                        sapid::field("showcase-button-size")
                            .theme(theme.clone())
                            .label("Size")
                            .control({
                                let theme = theme.clone();
                                move |field| {
                                    sapid::native_select("showcase-size")
                                        .theme(theme.clone())
                                        .field(field)
                                        .option(sapid::select_option("small", "Sm"))
                                        .option(sapid::select_option("medium", "Md"))
                                        .option(sapid::select_option("large", "Lg"))
                                        .value(size)
                                        .on_value_change(move |next| {
                                            if next.is_some() {
                                                size.revise(|v| *v = next);
                                            }
                                        })
                                }
                            }),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Disabled")
                            .checked(disabled)
                            .on_checked_change(move |next| disabled.revise(|v| *v = next)),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Loading")
                            .checked(loading)
                            .on_checked_change(move |next| loading.revise(|v| *v = next)),
                    )),
            ))
            .show_in(scope)
    })
}

#[allow(dead_code, unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(208.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
        )
    }

    pub fn checkbox(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::checkbox()
                .theme(theme.clone())
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn size(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        select(
            "size",
            "Size",
            value,
            &[
                ("xs", "Extra small"),
                ("sm", "Small"),
                ("md", "Medium"),
                ("lg", "Large"),
                ("xl", "Extra large"),
            ],
            theme,
        )
    }

    pub fn size_value(value: &str) -> Size {
        match value {
            "xs" => Size::ExtraSmall,
            "sm" => Size::Small,
            "lg" => Size::Large,
            "xl" => Size::ExtraLarge,
            _ => Size::Medium,
        }
    }

    const COLORS: &[(&str, Color)] = &[
        ("Yellow", Color(250, 176, 5, 255)),
        ("Orange", Color(253, 126, 20, 255)),
        ("Red", Color(250, 82, 82, 255)),
        ("Pink", Color(230, 73, 128, 255)),
        ("Grape", Color(190, 75, 219, 255)),
        ("Violet", Color(121, 80, 242, 255)),
        ("Indigo", Color(76, 110, 245, 255)),
        ("Blue", Color(34, 139, 230, 255)),
        ("Cyan", Color(21, 170, 191, 255)),
        ("Teal", Color(18, 184, 134, 255)),
        ("Green", Color(64, 192, 87, 255)),
        ("Gray", Color(134, 142, 150, 255)),
    ];

    pub fn accent(mut theme: Theme, name: &str) -> Theme {
        if let Some((_, color)) = COLORS.iter().find(|(label, _)| *label == name) {
            theme.accent = *color;
            theme.accent_text = *color;
            theme.focus_ring = *color;
            theme.input_focus = *color;
        }
        theme
    }

    pub fn color_choices(selected: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Color"),
                sapid::group()
                    .theme(theme.clone())
                    .class("site-control-swatches")
                    .fill(
                        COLORS
                            .iter()
                            .map(|(name, color)| {
                                let active = selected.map(move |current| {
                                    if current == *name {
                                        "site-control-swatch is-active".to_owned()
                                    } else {
                                        "site-control-swatch".to_owned()
                                    }
                                });
                                sapid::button()
                                    .unstyled()
                                    .class(active)
                                    .name(format!("Use {name}"))
                                    .label("")
                                    .style(Style {
                                        background: Some(*color),
                                        ..Style::new()
                                    })
                                    .on_press(move || {
                                        selected.revise(|current| *current = (*name).to_owned())
                                    })
                            })
                            .collect::<Vec<_>>(),
                    ),
            )),
        )
    }
}
