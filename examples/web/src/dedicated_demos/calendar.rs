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
                sapid::calendar("project-calendar", provider())
                    .theme(theme.clone())
                    .name("Project review calendar")
                    .value(value)
                    .with_week_numbers(true)
                    .on_value_change(move |v| value.revise(|s| *s = v)),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.map(|d| format!("Review scheduled for {d}"))
                        .unwrap_or_else(|| "Choose a day for your review.".to_owned())
                })),
            ))
            .show_in(scope)
    })
}

fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap())
        .today(Some(CivilDate::parse_iso("2026-11-04").unwrap()))
}
