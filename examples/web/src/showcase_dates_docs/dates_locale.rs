use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::date_picker(
                "localized-review-picker",
                provider().locale(DateLocale::ZhCn),
            )
            .theme(theme.clone())
            .name("Localized date picker"),
            sapid::date_picker(
                "sunday-review-picker",
                provider().week_start(Weekday::Sunday),
            )
            .theme(theme.clone())
            .name("Sunday-first date picker"),
        ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
