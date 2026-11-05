use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accept = scope.cage(false);
        let value = scope.cage(Some(day("2026-11-04")));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow appointment date changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|s| *s = v)),
                sapid::field("controlled-editable-date")
                    .theme(theme.clone())
                    .label("Controlled appointment date")
                    .control(move |field| {
                        sapid::date_input("controlled-editable-date", provider())
                            .field(field)
                            .value(value)
                            .on_value_change(move |v| {
                                if accept.get() {
                                    value.revise(|s| *s = v)
                                }
                            })
                    }),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.map(|d| format!("Saved appointment: {d}"))
                        .unwrap_or_else(|| "The saved appointment is empty.".to_owned())
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
