use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::inline_date_time_picker("inline-bounded-review", provider(), DateTimePolicy::Civil)
        .theme(theme.clone())
        .name("Bounded inline review")
        .bounds(
            DateTimeBounds::new(Some(dt("2026-11-05T10:00")), Some(dt("2026-11-08T17:00")))
                .unwrap(),
        )
        .default_time(clock("10:30"))
        .default_value(Some(dt("2026-11-06T10:30")))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn clock(value: &str) -> LocalTime {
    LocalTime::parse_iso(value).unwrap()
}
fn dt(value: &str) -> LocalDateTime {
    sapid::parse_local_date_time(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
