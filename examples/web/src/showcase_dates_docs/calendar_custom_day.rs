use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<CivilDate>);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(16.)
            .fill((
                sapid::calendar("custom-review-days", provider())
                    .theme(theme.clone())
                    .name("Review calendar with event markers")
                    .value(value)
                    .on_value_change(move |v| value.revise(|s| *s = v))
                    .render_day(|date| {
                        widget(move |scope| {
                            let date = date.clone();
                            glory::ui::text()
                                .content(scope.bond(move || {
                                    let d = date.get();
                                    if d.day() == 16 {
                                        format!("{} •", d.day())
                                    } else {
                                        d.day().to_string()
                                    }
                                }))
                                .show_in(scope)
                        })
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content("The dot marks the team review on November 16."),
            ))
            .show_in(scope)
    })
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
