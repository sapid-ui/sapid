use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P1-03", theme)
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
    use sapid::Theme;

    pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
        match id {
            "P1-01" => Box::new(widget(move |scope| {
                let loading = scope.cage(true);
                let invalid = scope.cage(false);
                let error = scope.bond(move || {
                    if invalid.get() {
                        "That workspace name is already used".to_owned()
                    } else {
                        String::new()
                    }
                });
                sapid::stack()
                    .theme(theme.clone())
                    .gap(18.)
                    .fill((
                        sapid::field("focused-field-email")
                            .theme(theme.clone())
                            .label("Email address")
                            .description("We will only use this for important account updates.")
                            .required(true)
                            .control(|field| {
                                sapid::text_field()
                                    .field(field)
                                    .default_value("alex@studio.design")
                            }),
                        sapid::field("focused-field-sections")
                            .theme(theme.clone())
                            .label("Workspace URL")
                            .description("Loading and both sections belong to the real input.")
                            .control({
                                let theme = theme.clone();
                                move |field| {
                                    sapid::text_field()
                                        .field(field)
                                        .default_value("design")
                                        .loading(loading)
                                        .loading_position(sapid::InputSectionPosition::Right)
                                        .left_section(
                                            sapid::text().theme(theme.clone()).content("@"),
                                        )
                                        .right_section_width(44.)
                                }
                            }),
                        sapid::group().theme(theme.clone()).gap(8.).fill(
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Secondary)
                                .label(scope.bond(move || {
                                    if loading.get() {
                                        "Stop loading".to_owned()
                                    } else {
                                        "Start loading".to_owned()
                                    }
                                }))
                                .on_press(move || loading.revise(|value| *value = !*value)),
                        ),
                        sapid::field("focused-field-success")
                            .theme(theme.clone())
                            .label("Workspace name")
                            .description(
                                "The description is intentionally rendered before the label.",
                            )
                            .error(error)
                            .success(true)
                            .order([
                                sapid::FieldSlot::Description,
                                sapid::FieldSlot::Label,
                                sapid::FieldSlot::Control,
                                sapid::FieldSlot::Error,
                            ])
                            .control(|field| {
                                sapid::text_field().field(field).default_value("north-star")
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .variant(sapid::ButtonVariant::Secondary)
                            .label(scope.bond(move || {
                                if invalid.get() {
                                    "Show success".to_owned()
                                } else {
                                    "Show error".to_owned()
                                }
                            }))
                            .on_press(move || invalid.revise(|value| *value = !*value)),
                    ))
                    .show_in(scope)
            })),
            "P1-02" => Box::new(
                sapid::field("focused-field-parts")
                    .theme(theme.clone())
                    .label("Work email")
                    .description("Use the address associated with your workspace.")
                    .error("Enter a valid email address, including the @ sign.")
                    .required(true)
                    .control(|field| {
                        sapid::text_field()
                            .field(field)
                            .default_value("alex@studio")
                    }),
            ),
            "P1-03" => Box::new(widget(move |scope| {
                let disabled = scope.cage(false);
                sapid::stack()
                    .theme(theme.clone())
                    .gap(16.0)
                    .fill((
                        sapid::fieldset("focused-profile")
                            .theme(theme.clone())
                            .legend("Personal information")
                            .disabled(disabled)
                            .fill((
                                sapid::field("focused-profile-name")
                                    .theme(theme.clone())
                                    .label("Your name")
                                    .control(|field| {
                                        sapid::text_field()
                                            .field(field)
                                            .default_value("Ada Lovelace")
                                    }),
                                sapid::field("focused-profile-email")
                                    .theme(theme.clone())
                                    .label("Email")
                                    .control(|field| {
                                        sapid::text_field()
                                            .field(field)
                                            .default_value("ada@example.com")
                                    }),
                            )),
                        sapid::group().theme(theme.clone()).fill(
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Secondary)
                                .size(sapid::Size::Small)
                                .label("Toggle fieldset disabled")
                                .on_press(move || disabled.revise(|value| *value = !*value)),
                        ),
                    ))
                    .show_in(scope)
            })),
            "P1-04" => Box::new(widget(move |scope| {
                let value = scope.cage(
                    "Tell us a little about yourself.\nThis textarea grows with its content."
                        .to_owned(),
                );
                let size = scope.cage(Some("md".to_owned()));
                let min_rows = scope.cage(Some("3".to_owned()));
                let max_rows = scope.cage(Some("6".to_owned()));
                let autosize = scope.cage(true);
                let readonly = scope.cage(false);
                let disabled = scope.cage(false);
                let enabled = scope.bond(move || !disabled.get());
                let revision = scope.bond(move || {
                    vec![format!(
                        "{}-{}-{}-{}",
                        size.get().unwrap_or_default(),
                        min_rows.get().unwrap_or_default(),
                        max_rows.get().unwrap_or_default(),
                        autosize.get()
                    )]
                });
                let preview_theme = theme.clone();
                let preview =
                    each(revision)
                        .key(|revision| revision.clone())
                        .children(move |entry| {
                            let size = super::controls::size_value(&entry.value.get());
                            let min_rows = min_rows
                                .get()
                                .and_then(|value| value.parse::<u16>().ok())
                                .unwrap_or(3);
                            let max_rows = max_rows
                                .get()
                                .and_then(|value| value.parse::<u16>().ok())
                                .unwrap_or(6);
                            sapid::field("focused-biography")
                                .theme(preview_theme.clone())
                                .label("Biography")
                                .description("Autosizes from three to six lines as you write.")
                                .readonly(readonly)
                                .enabled(enabled)
                                .control({
                                    let theme = preview_theme.clone();
                                    move |field| {
                                        sapid::textarea()
                                            .theme(theme.clone())
                                            .size(size)
                                            .field(field)
                                            .autosize(autosize.get())
                                            .min_rows(min_rows)
                                            .max_rows(max_rows)
                                            .value(value)
                                            .on_value_change(move |next| {
                                                value.revise(|v| *v = next)
                                            })
                                    }
                                })
                        });
                let controls = vec![
                    super::controls::size(size, &theme),
                    super::controls::select(
                        "min-rows",
                        "Minimum rows",
                        min_rows,
                        &[("2", "2"), ("3", "3"), ("4", "4")],
                        &theme,
                    ),
                    super::controls::select(
                        "max-rows",
                        "Maximum rows",
                        max_rows,
                        &[("4", "4"), ("5", "5"), ("6", "6")],
                        &theme,
                    ),
                    super::controls::checkbox("Autosize", autosize, &theme),
                    super::controls::checkbox("Read only", readonly, &theme),
                    super::controls::checkbox("Disabled", disabled, &theme),
                ];
                super::controls::playground(Box::new(preview), controls).show_in(scope)
            })),
            "P1-05" => Box::new(
                sapid::field("focused-password")
                    .theme(theme.clone())
                    .label("Account password")
                    .description("Use the visibility action without losing your input.")
                    .required(true)
                    .control(|field| {
                        sapid::password_input()
                            .field(field)
                            .default_value("example-password")
                    }),
            ),
            "P1-06" => Box::new(
                sapid::field("focused-website")
                    .theme(theme.clone())
                    .label("Website")
                    .description("Addons and actions stay aligned with the input.")
                    .control(move |field| {
                        sapid::input_group()
                            .theme(theme.clone())
                            .start(
                                sapid::input_addon()
                                    .theme(theme.clone())
                                    .content("https://"),
                            )
                            .input(
                                sapid::text_field()
                                    .field(field)
                                    .default_value("studio.design"),
                            )
                            .end(
                                sapid::input_section().theme(theme.clone()).fill(
                                    sapid::button()
                                        .theme(theme.clone())
                                        .variant(sapid::ButtonVariant::Secondary)
                                        .size(sapid::Size::Small)
                                        .label("Check"),
                                ),
                            )
                    }),
            ),
            _ => unreachable!("unsupported fields demo"),
        }
    }
}
