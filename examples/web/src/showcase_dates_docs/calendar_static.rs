use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::calendar("static-review-calendar", provider())
                .theme(theme.clone())
                .name("Static November calendar")
                .static_mode(true),
            sapid::calendar("quiet-review-calendar", provider())
                .theme(theme.clone())
                .name("Quiet November calendar")
                .hide_weekdays(true)
                .hide_outside_dates(true),
        ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
