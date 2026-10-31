use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P3-08", theme)
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

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::{KbdKey, KbdPlatform, Theme};

    pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
        match id {
            "P3-07" => {
                Box::new(
                    sapid::text_list()
                        .theme(theme.clone())
                        .ordered(true)
                        .name("Getting started")
                        .item(sapid::text_list_item().fill(text().content("Create an application")))
                        .item(sapid::text_list_item().fill((
                            text().content("Compose widgets"),
                            sapid::text_list().theme(theme.clone()).item(
                                sapid::text_list_item().fill(text().content("Keep keys stable")),
                            ),
                        )))
                        .item(sapid::text_list_item().fill(text().content("Choose a host"))),
                )
            }
            "P3-08" => Box::new(
                sapid::blockquote()
                    .theme(theme.clone())
                    .fill(text().content(
                        "Make interfaces simpler by giving each piece of UI a clear semantic role.",
                    ))
                    .citation(text().content("Sapid design principles")),
            ),
            "P3-09" => Box::new(
                sapid::stack().theme(theme.clone()).gap(18.0).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .content("Inline code: let glory = 42;"),
                    sapid::code()
                        .theme(theme.clone())
                        .content("let glory = 42;"),
                    sapid::mark().theme(theme.clone()).content("Marked text"),
                    sapid::highlight()
                        .theme(theme.clone())
                        .content("Highlight only the important word in this sentence.")
                        .query("important"),
                )),
            ),
            "P3-16" => Box::new(
                sapid::group().theme(theme.clone()).fill((
                    sapid::kbd()
                        .theme(theme.clone())
                        .platform(KbdPlatform::Mac)
                        .key(KbdKey::Meta),
                    sapid::text().theme(theme.clone()).content("+"),
                    sapid::kbd().theme(theme.clone()).content("K"),
                    sapid::text().theme(theme.clone()).content("to open search"),
                )),
            ),
            "P3-18" => Box::new(
                sapid::stack().theme(theme.clone()).gap(20.0).fill((
                    sapid::item()
                        .theme(theme.clone())
                        .title("Sapid")
                        .description("Composable semantic widgets for Glory")
                        .media(sapid::badge().theme(theme.clone()).content("RS")),
                    sapid::data_list()
                        .theme(theme.clone())
                        .name("Project metadata")
                        .item(
                            sapid::data_list_item()
                                .term("Package")
                                .fill(text().content("sapid")),
                        )
                        .item(
                            sapid::data_list_item()
                                .term("Host")
                                .fill(text().content("Web, SSR and native")),
                        ),
                )),
            ),
            _ => unreachable!("unsupported document demo"),
        }
    }
}
