use glory::prelude::*;
use sapid::{
    DateTimePolicy, DateTimeRangeValue, DatesProvider, LocalTime, inline_date_time_range_picker,
    parse_local_date_time,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    let conference = DateTimeRangeValue::new(
        Some(value("2026-11-04T09:00")),
        Some(value("2026-11-06T17:30")),
    )
    .unwrap();
    inline_date_time_range_picker(
        "showcase-inline-date-time-range",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Inline date and time range")
    .default_times(
        LocalTime::new(9, 0, 0).unwrap(),
        LocalTime::new(17, 30, 0).unwrap(),
    )
    .default_value(conference)
}
