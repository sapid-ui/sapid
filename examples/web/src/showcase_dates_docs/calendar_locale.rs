use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::calendar(
                "sunday-review-calendar",
                provider().week_start(Weekday::Sunday),
            )
            .theme(theme.clone())
            .name("Sunday-first review calendar"),
            sapid::calendar(
                "localized-review-calendar",
                provider()
                    .locale(DateLocale::ZhCn)
                    .week_start(Weekday::Monday),
            )
            .theme(theme.clone())
            .name("Localized review calendar"),
        ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
