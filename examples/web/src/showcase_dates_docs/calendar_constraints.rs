use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::calendar(
        "constrained-review-calendar",
        provider()
            .bounds(DateBounds::new(Some(day("2026-11-05")), Some(day("2026-12-15"))).unwrap()),
    )
    .theme(theme.clone())
    .name("Available review dates")
    .disable_weekday(Weekday::Saturday)
    .disable_weekday(Weekday::Sunday)
    .exclude_date(day("2026-11-16"))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
