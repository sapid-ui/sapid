use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P7-01", theme)
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
                                .width(Length::Px(240.0))
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

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn text_field(label: &str, value: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::field(format!("control-{label}"))
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    sapid::text_field()
                        .field(field)
                        .placeholder("Enter prop value")
                        .value(value)
                        .on_value_change(move |next| value.revise(|current| *current = next))
                }),
        )
    }

    pub fn size_slider(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        let current = value.map(|current| match current.as_deref() {
            Some("xs") => 0.0,
            Some("sm") => 1.0,
            Some("lg") => 3.0,
            Some("xl") => 4.0,
            _ => 2.0,
        });
        let value_text = value.map(|current| {
            Some(
                match current.as_deref() {
                    Some("xs") => "Extra small",
                    Some("sm") => "Small",
                    Some("lg") => "Large",
                    Some("xl") => "Extra large",
                    _ => "Medium",
                }
                .to_owned(),
            )
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Size"),
                sapid::slider("control-size")
                    .theme(theme.clone())
                    .name("Size")
                    .min(0.0)
                    .max(4.0)
                    .step(1.0)
                    .value(current)
                    .value_text(value_text)
                    .on_value_change(move |next| {
                        value.revise(|current| {
                            *current = Some(
                                match next.round() as u8 {
                                    0 => "xs",
                                    1 => "sm",
                                    3 => "lg",
                                    4 => "xl",
                                    _ => "md",
                                }
                                .to_owned(),
                            )
                        })
                    }),
            )),
        )
    }

    pub fn radius_slider(value: Cage<f64>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Radius"),
                sapid::slider("control-radius")
                    .theme(theme.clone())
                    .name("Radius")
                    .min(0.0)
                    .max(12.0)
                    .step(1.0)
                    .value(value)
                    .value_text(value.map(|value| Some(format!("{value} px"))))
                    .on_value_change(move |next| value.revise(|current| *current = next)),
            )),
        )
    }

    pub fn variant(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        let current = value.map(|value| value.as_deref().unwrap_or("default").to_owned());
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Variant"),
                sapid::segmented_control("control-variant")
                    .theme(theme.clone())
                    .class("site-variant-control")
                    .name("Variant")
                    .value(current)
                    .segments([
                        sapid::segment("default").label("Default"),
                        sapid::segment("filled").label("Filled"),
                        sapid::segment("unstyled").label("Unstyled"),
                    ])
                    .on_value_change(move |next| value.revise(|current| *current = Some(next))),
            )),
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

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::{Decimal, NumberValue, Theme};

    pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
        assert_eq!(id, "P7-01");
        Box::new(widget(move |scope| {
            let value = scope.cage(NumberValue::Decimal(
                Decimal::parse("1234.5").expect("valid showcase number"),
            ));
            let variant = scope.cage(Some("default".to_owned()));
            let size = scope.cage(Some("md".to_owned()));
            let radius = scope.cage(4.0);
            let label = scope.cage("Amount".to_owned());
            let description = scope.cage("Enter a value between 0 and 10,000.".to_owned());
            let error = scope.cage(String::new());
            let required = scope.cage(true);
            let grouping = scope.cage(true);
            let controls = scope.cage(true);
            let readonly = scope.cage(false);
            let disabled = scope.cage(false);
            let enabled = scope.bond(move || !disabled.get());
            let revision = scope.bond(move || {
                vec![format!(
                    "{}-{}-{}-{}-{}",
                    variant.get().unwrap_or_default(),
                    size.get().unwrap_or_default(),
                    radius.get(),
                    grouping.get(),
                    controls.get()
                )]
            });
            let preview_theme = theme.clone();
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |_| {
                    let size = super::controls::size_value(&size.get().unwrap_or_default());
                    let grouping = grouping.get();
                    let controls = controls.get();
                    let unstyled = variant.get().as_deref() == Some("unstyled");
                    let mut preview_theme = preview_theme.clone();
                    preview_theme.radius = radius.get();
                    if variant.get().as_deref() == Some("filled") {
                        preview_theme.input = preview_theme.disabled;
                    }
                    sapid::field("focused-number-amount")
                        .theme(preview_theme.clone())
                        .label(label)
                        .description(description)
                        .error(error)
                        .required(required)
                        .readonly(readonly)
                        .enabled(enabled)
                        .control(move |field| {
                            let mut input = sapid::number_input()
                                .field(field)
                                .size(size)
                                .value(value)
                                .min(Decimal::parse("0").ok())
                                .max(Decimal::parse("10000").ok())
                                .step(Decimal::parse("0.5").expect("valid showcase step"))
                                .grouping(grouping)
                                .controls(controls)
                                .on_value_change(move |next| {
                                    value.revise(|current| *current = next)
                                })
                                .on_commit(move |next| value.revise(|current| *current = next));
                            if unstyled {
                                input = input.unstyled();
                            }
                            input
                        })
                });
            let controls = vec![
                super::controls::variant(variant, &theme),
                super::controls::size_slider(size, &theme),
                super::controls::radius_slider(radius, &theme),
                super::controls::text_field("Label", label, &theme),
                super::controls::switch("With asterisk", required, &theme),
                super::controls::text_field("Description", description, &theme),
                super::controls::text_field("Error", error, &theme),
                super::controls::switch("Thousand separator", grouping, &theme),
                super::controls::switch("Step controls", controls, &theme),
                super::controls::switch("Read only", readonly, &theme),
                super::controls::switch("Disabled", disabled, &theme),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        }))
    }
}
