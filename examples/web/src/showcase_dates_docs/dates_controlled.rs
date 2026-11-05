use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accept = scope.cage(false);
        let value = scope.cage(DateSelection::Single(None));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow review date changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|s| *s = v)),
                sapid::date_picker("controlled-review-picker", provider())
                    .theme(theme.clone())
                    .name("Controlled review date")
                    .value(value)
                    .allow_deselect(true)
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|s| *s = v)
                        }
                    }),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    let s = v.format_iso();
                    if s.is_empty() {
                        "No review date has been saved.".to_owned()
                    } else {
                        format!("Saved review date: {s}")
                    }
                })),
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
