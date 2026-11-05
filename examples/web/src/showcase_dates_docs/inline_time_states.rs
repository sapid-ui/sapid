use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::inline_date_time_picker(
                "inline-readonly-review",
                provider(),
                DateTimePolicy::Civil,
            )
            .theme(theme.clone())
            .name("Readonly inline review")
            .readonly(true)
            .default_value(Some(dt("2026-11-04T10:30"))),
            sapid::inline_date_time_picker(
                "inline-disabled-review",
                provider(),
                DateTimePolicy::Civil,
            )
            .theme(theme.clone())
            .name("Disabled inline review")
            .enabled(false)
            .default_value(Some(dt("2026-11-04T10:30"))),
        ))
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
