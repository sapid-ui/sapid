use glory::prelude::*;
use sapid::{ButtonVariant, MultiSelectHandle, Theme, select_option};
use std::{cell::RefCell, rc::Rc};

pub fn demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let values = scope.cage(vec![
            "ca".to_owned(),
            "ic".to_owned(),
            "fr".to_owned(),
            "de".to_owned(),
        ]);
        let handle = Rc::new(RefCell::new(None::<MultiSelectHandle>));
        let ready = handle.clone();
        let all = handle.clone();
        let clear = handle.clone();
        let enabled = scope.cage(true);
        let readonly = scope.cage(false);
        let commands_enabled = scope.bond(move || enabled.get() && !readonly.get());
        let weak_all = scope.downgrade();
        let weak_clear = scope.downgrade();
        let status = scope.bond(move || format!("{} countries selected", values.with(Vec::len)));
        sapid::stack()
            .theme(theme.clone())
            .gap(18.0)
            .fill((
                sapid::field("showcase-countries")
                    .theme(theme.clone())
                    .label("Countries")
                    .description("Search, choose several, or remove a tag.")
                    .control(move |field| {
                        let ready = ready.clone();
                        sapid::multi_select("showcase-countries")
                            .field(field)
                            .value(values)
                            .max_values(4)
                            .clearable(true)
                            .clear_label("Clear countries")
                            .placeholder("Search countries")
                            .enabled(enabled)
                            .readonly(readonly)
                            .options([
                                select_option("ca", "Canada"),
                                select_option("ic", "Canary Islands"),
                                select_option("fr", "France"),
                                select_option("de", "Germany"),
                                select_option("jp", "Japan").disabled(true),
                            ])
                            .on_value_change(move |proposal| {
                                values.revise(|current| *current = proposal)
                            })
                            .on_ready(move |command| *ready.borrow_mut() = Some(command))
                    }),
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .variant(ButtonVariant::Secondary)
                        .label("Select filtered countries")
                        .enabled(commands_enabled)
                        .on_press(move || {
                            let command = all.borrow().clone();
                            if let Some(command) = command
                                && let Err(error) = command.select_all()
                                && let Some(scope) = weak_all.upgrade()
                            {
                                scope.report_error(error);
                            }
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .variant(ButtonVariant::Secondary)
                        .label("Clear selected countries")
                        .enabled(commands_enabled)
                        .on_press(move || {
                            let command = clear.borrow().clone();
                            if let Some(command) = command
                                && let Err(error) = command.clear_all()
                                && let Some(scope) = weak_clear.upgrade()
                            {
                                scope.report_error(error);
                            }
                        }),
                )),
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Small)
                    .content(status),
                sapid::group().theme(theme.clone()).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Disabled")
                        .checked(enabled.map(|value| !value))
                        .on_checked_change(move |value| {
                            enabled.revise(|current| *current = !value)
                        }),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Read only")
                        .checked(readonly)
                        .on_checked_change(move |value| {
                            readonly.revise(|current| *current = value)
                        }),
                )),
                limited_choices(theme.clone()),
            ))
            .show_in(scope)
    })
}

fn limited_choices(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let values = scope.cage(vec!["rust".to_owned()]);
        sapid::field("showcase-tools")
            .theme(theme.clone())
            .label("Tools")
            .description(
                "Select up to two tools. Selected options are hidden; clearing keeps your search.",
            )
            .control(move |field| {
                sapid::multi_select("showcase-tools")
                    .field(field)
                    .value(values)
                    .max_values(2)
                    .hide_picked_options(true)
                    .clearable(true)
                    .clear_label("Clear tools")
                    .placeholder("Search tools")
                    .group(
                        sapid::select_group("Runtime")
                            .option(select_option("rust", "Rust"))
                            .option(select_option("glory", "Glory")),
                    )
                    .group(sapid::select_group("Design").option(select_option("sapid", "Sapid")))
                    .on_value_change(move |proposal| values.revise(|current| *current = proposal))
            })
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
