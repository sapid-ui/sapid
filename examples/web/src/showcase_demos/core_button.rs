use glory::prelude::*;
use glory::style::FlexWrap;
use glory::ui::IntoChildren;

fn column() -> glory::ui::View {
    col().style(
        Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .gap(16.0),
    )
}

fn choice(
    theme: sapid::Theme,
    key: &'static str,
    label: &'static str,
    value: Cage<Option<String>>,
    options: &'static [(&'static str, &'static str)],
) -> impl Widget {
    sapid::field(key)
        .theme(theme.clone())
        .label(label)
        .control(move |field| {
            let mut select = sapid::native_select(format!("{key}-control"))
                .theme(theme.clone())
                .field(field);
            for (option, text) in options {
                select = select.option(sapid::select_option(*option, *text));
            }
            select.value(value).on_value_change(move |next| {
                if next.is_some() {
                    value.revise(|current| *current = next);
                }
            })
        })
}

fn scenario(
    theme: sapid::Theme,
    title: &'static str,
    content: impl IntoChildren,
) -> glory::ui::View {
    column().class("site-button-scenario").fill((
        sapid::title(3).theme(theme).content(title),
        view().class("site-button-scenario-content").fill(content),
    ))
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let clicks = scope.cage(0usize);
        let variant = scope.cage(Some("filled".to_owned()));
        let color = scope.cage(Some("accent".to_owned()));
        let size = scope.cage(Some("medium".to_owned()));
        let radius = scope.cage(Some("small".to_owned()));
        let compact = scope.cage(false);
        let disabled = scope.cage(false);
        let loading = scope.cage(false);
        let style_theme = theme.clone();
        let button_style = scope.bond(move || {
            let size = match size.get().as_deref() {
                Some("extra-small") => sapid::Size::ExtraSmall,
                Some("small") => sapid::Size::Small,
                Some("large") => sapid::Size::Large,
                Some("extra-large") => sapid::Size::ExtraLarge,
                _ => sapid::Size::Medium,
            };
            let control = style_theme.tokens.control(size);
            let text = style_theme
                .tokens
                .typography(sapid::TypographyRole::Control, size);
            let (fill, on_fill) = match color.get().as_deref() {
                Some("success") => (style_theme.success, style_theme.on_accent),
                Some("danger") => (style_theme.danger_fill, style_theme.on_danger),
                _ => (style_theme.accent, style_theme.on_accent),
            };
            let (background, foreground, border) = if disabled.get() || loading.get() {
                (
                    style_theme.disabled,
                    style_theme.disabled_text,
                    style_theme.border,
                )
            } else {
                match variant.get().as_deref() {
                    Some("outline") => (style_theme.input, fill, fill),
                    Some("light") => (style_theme.accent_soft, fill, style_theme.accent_soft),
                    Some("subtle") => (style_theme.input, fill, style_theme.input),
                    _ => (fill, on_fill, fill),
                }
            };
            let padding_y = if compact.get() {
                control.padding_y / 2.0
            } else {
                control.padding_y
            };
            let radius = match radius.get().as_deref() {
                Some("extra-small") => 2.0,
                Some("medium") => 8.0,
                Some("large") => 16.0,
                _ => 4.0,
            };
            Style {
                background: Some(background),
                color: Some(foreground),
                border_color: Some(border),
                font_size: Some(text.font_size),
                line_height: Some(text.line_height),
                font_weight: text.weight,
                ..Style::new()
                    .min_height(Length::Px(if compact.get() {
                        control.min_height - 8.0
                    } else {
                        control.min_height
                    }))
                    .padding(Edges {
                        top: padding_y,
                        bottom: padding_y,
                        left: control.padding_x,
                        right: control.padding_x,
                    })
                    .radius(radius)
            }
        });
        let label = scope.bond(move || {
            if loading.get() {
                return "Loading…".to_owned();
            }
            match variant.get().as_deref() {
                Some("outline") => "Outline button",
                Some("light") => "Light button",
                Some("subtle") => "Subtle button",
                _ => "Filled button",
            }
            .to_owned()
        });
        let playground = view()
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
                            .compact(compact)
                            .enabled(disabled.map(|value| !value))
                            .loading(loading)
                            .on_press(move || clicks.revise(|value| *value += 1)),
                        sapid::text()
                            .theme(theme.clone())
                            .content(clicks.map(|value| format!("Clicked {value} times"))),
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
                                    .on_press(move || clicks.revise(|value| *value += 1))
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
                        choice(
                            theme.clone(),
                            "showcase-button-variant",
                            "Variant",
                            variant,
                            &[
                                ("filled", "Filled"),
                                ("light", "Light"),
                                ("outline", "Outline"),
                                ("subtle", "Subtle"),
                            ],
                        ),
                        choice(
                            theme.clone(),
                            "showcase-button-color",
                            "Color",
                            color,
                            &[
                                ("accent", "Accent"),
                                ("success", "Success"),
                                ("danger", "Danger"),
                            ],
                        ),
                        choice(
                            theme.clone(),
                            "showcase-button-size",
                            "Size",
                            size,
                            &[
                                ("extra-small", "Xs"),
                                ("small", "Sm"),
                                ("medium", "Md"),
                                ("large", "Lg"),
                                ("extra-large", "Xl"),
                            ],
                        ),
                        choice(
                            theme.clone(),
                            "showcase-button-radius",
                            "Radius",
                            radius,
                            &[
                                ("extra-small", "Xs"),
                                ("small", "Sm"),
                                ("medium", "Md"),
                                ("large", "Lg"),
                            ],
                        ),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Compact")
                            .checked(compact)
                            .on_checked_change(move |next| compact.revise(|value| *value = next)),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Disabled")
                            .checked(disabled)
                            .on_checked_change(move |next| disabled.revise(|value| *value = next)),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Loading")
                            .checked(loading)
                            .on_checked_change(move |next| loading.revise(|value| *value = next)),
                    )),
            ));
        view()
            .class("site-button-examples")
            .fill((
                playground,
                scenario(
                    theme.clone(),
                    "Full width",
                    sapid::button()
                        .theme(theme.clone())
                        .label("Full width button")
                        .style(Style::new().width(Length::Percent(100.0))),
                ),
                scenario(
                    theme.clone(),
                    "Left and right sections",
                    (
                        sapid::button()
                            .theme(theme.clone())
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("Gallery")
                            .left_section(text().content("▧")),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Download")
                            .right_section(text().content("↓")),
                        sapid::button()
                            .theme(theme.clone())
                            .variant(sapid::ButtonVariant::Light)
                            .label("Visit gallery")
                            .left_section(text().content("▧"))
                            .right_section(text().content("→")),
                    ),
                ),
                scenario(
                    theme.clone(),
                    "Sections position",
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Secondary)
                        .label("Button label")
                        .left_section(text().content("▧"))
                        .right_section(text().content("→"))
                        .style(Style {
                            justify: Some(Justify::Between),
                            ..Style::new().width(Length::Percent(100.0))
                        }),
                ),
                scenario(
                    theme.clone(),
                    "Compact size",
                    (
                        sapid::button().theme(theme.clone()).label("Regular md"),
                        sapid::button()
                            .theme(theme.clone())
                            .compact(true)
                            .label("Compact md"),
                    ),
                ),
                scenario(
                    theme.clone(),
                    "Disabled state",
                    sapid::button()
                        .theme(theme.clone())
                        .label("Disabled button")
                        .enabled(false),
                ),
                scenario(
                    theme.clone(),
                    "Loading state",
                    sapid::button()
                        .theme(theme.clone())
                        .label("Loading button")
                        .loading(true),
                ),
                scenario(
                    theme.clone(),
                    "Button.Group",
                    sapid::button_group()
                        .theme(theme.clone())
                        .name("Button group")
                        .buttons([
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Secondary)
                                .label("First"),
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Secondary)
                                .label("Second"),
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Secondary)
                                .label("Third"),
                        ]),
                ),
            ))
            .show_in(scope)
    })
}
