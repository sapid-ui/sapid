use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::inline_date_time_picker("inline-seconds-review", provider(), DateTimePolicy::Civil)
        .theme(theme.clone())
        .name("Inline seconds review")
        .format(TimeFormat::new(
            HourCycle::H12,
            TimePrecision::Second,
            DateLocale::En,
        ))
        .default_time(clock("09:30:45"))
        .default_value(Some(dt("2026-11-04T09:30:45")))
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
