use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Calendar", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{FileConfig, Orientation};
    use glory::style::{FlexWrap, ObjectFit};
    use glory::ui::text;
    use sapid::prelude::*;
    use sapid::{
        Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button,
        file_input, group, image, indicator, progress, ring_progress, select_group, select_option,
        semi_circle_progress, theme_icon,
    };
    use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

    fn date(value: &str) -> CivilDate {
        CivilDate::parse_iso(value).unwrap()
    }

    fn documentation_heading(theme: &Theme, id: &'static str, title: &'static str) -> impl Widget {
        glory::ui::view()
            .semantics(Some(Box::new(glory::render::Semantics {
                id: Some(glory::render::SemanticId::new(id).expect("showcase section id")),
                ..Default::default()
            })))
            .fill(sapid::title(3).theme(theme.clone()).content(title))
    }

    fn calendar_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let provider = DatesProvider::new(date("2024-02-29"));
            let custom = scope.cage(DateSelection::Multiple(SelectedDates::new()));
            let selected = custom;
            let summary = custom.map(|selection| match selection {
                DateSelection::Multiple(values) => format!("Selected: {} / 3", values.len()),
                _ => unreachable!(),
            });
            let rendered_day = |date: Vary<CivilDate>| {
                widget(move |scope| {
                    let date = date.clone();
                    let value = scope.bond(move || {
                        let date = date.get();
                        if date.day() == 16 {
                            format!("{} •", date.day())
                        } else {
                            date.day().to_string()
                        }
                    });
                    glory::ui::text()
                        .class("site-calendar-rendered-day")
                        .content(value)
                        .show_in(scope)
                })
            };
            let children: Vec<Box<dyn Widget>> = vec![
                Box::new(documentation_heading(&theme, "calendar-usage", "Usage")),
                Box::new(
                    calendar("calendar-usage-grid", provider)
                        .theme(theme.clone())
                        .name("Calendar usage")
                        .default_month(YearMonth::new(2024, 2).unwrap()),
                ),
                Box::new(documentation_heading(
                    &theme,
                    "calendar-custom-picker",
                    "Custom date picker",
                )),
                Box::new(
                    calendar("calendar-custom-grid", provider)
                        .theme(theme.clone())
                        .name("Pick up to three dates")
                        .default_month(YearMonth::new(2024, 2).unwrap())
                        .selection(custom)
                        .on_value_change(move |date| {
                            let Some(date) = date else { return };
                            selected.revise(|selection| {
                                let DateSelection::Multiple(mut values) = *selection else {
                                    unreachable!()
                                };
                                if !values.remove(date) && values.len() < 3 {
                                    values
                                        .insert(date)
                                        .expect("showcase selection stays bounded");
                                }
                                *selection = DateSelection::Multiple(values);
                            });
                        }),
                ),
                Box::new(sapid::text().theme(theme.clone()).content(summary)),
                Box::new(documentation_heading(
                    &theme,
                    "calendar-full-width",
                    "Full width",
                )),
                Box::new(
                    calendar("calendar-full-width-grid", provider)
                        .theme(theme.clone())
                        .name("Full width calendar")
                        .default_month(YearMonth::new(2024, 2).unwrap())
                        .full_width(true),
                ),
                Box::new(documentation_heading(
                    &theme,
                    "calendar-static",
                    "Static calendar and custom days",
                )),
                Box::new(
                    calendar("calendar-static-grid", provider)
                        .theme(theme.clone())
                        .name("Static event calendar")
                        .default_month(YearMonth::new(2024, 2).unwrap())
                        .static_mode(true)
                        .render_day(rendered_day),
                ),
                Box::new(documentation_heading(
                    &theme,
                    "calendar-options",
                    "Week numbers and weekday header",
                )),
                Box::new(
                    sapid::group().theme(theme.clone()).fill((
                        calendar("calendar-weeks-grid", provider)
                            .theme(theme.clone())
                            .name("Calendar with week numbers")
                            .default_month(YearMonth::new(2024, 2).unwrap())
                            .with_week_numbers(true),
                        calendar("calendar-no-weekdays-grid", provider)
                            .theme(theme.clone())
                            .name("Calendar without weekday header")
                            .default_month(YearMonth::new(2024, 2).unwrap())
                            .hide_weekdays(true),
                    )),
                ),
            ];
            sapid::stack()
                .theme(theme.clone())
                .gap(24.0)
                .fill(children)
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Calendar" => Box::new(calendar_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
