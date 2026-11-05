use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::inline_date_time_picker("inline-preset-review", provider(), DateTimePolicy::Civil)
        .theme(theme.clone())
        .name("Inline preset review")
        .default_time(clock("09:00"))
        .default_value(Some(dt("2026-11-05T09:00")))
        .presets([
            DateTimePreset::new("Morning review", dt("2026-11-05T09:00")),
            DateTimePreset::new("Afternoon review", dt("2026-11-05T14:30")),
        ])
        .time_presets(vec![clock("09:00"), clock("10:30"), clock("14:30")])
        .time_dropdown(true)
        .time_steps(1, 15, 1)
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
