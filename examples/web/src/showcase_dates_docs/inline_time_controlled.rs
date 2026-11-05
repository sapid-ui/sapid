use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accept = scope.cage(false);
        let value = scope.cage(Some(dt("2026-11-04T09:30")));
        let revision = scope.cage(0u64);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow inline review changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|s| *s = v)),
                sapid::inline_date_time_picker(
                    "inline-controlled-review",
                    provider(),
                    DateTimePolicy::Civil,
                )
                .theme(theme.clone())
                .name("Controlled inline review")
                .value(value)
                .reset_revision(revision)
                .on_value_change(move |v| {
                    if accept.get() {
                        value.revise(|s| *s = v)
                    }
                }),
                sapid::button()
                    .theme(theme.clone())
                    .variant(ButtonVariant::Outline)
                    .label("Restore accepted review")
                    .on_press(move || revision.revise(|n| *n += 1)),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.map(|d| format!("Saved review: {}", sapid::format_local_date_time(d)))
                        .unwrap_or_else(|| "No review is saved.".to_owned())
                })),
            ))
            .show_in(scope)
    })
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn dt(value: &str) -> LocalDateTime {
    sapid::parse_local_date_time(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
