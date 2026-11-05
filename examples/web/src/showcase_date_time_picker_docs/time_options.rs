use glory::prelude::*;
use sapid::{
    DateLocale, DateTimePolicy, DatesProvider, HourCycle, TimeFormat, TimePrecision,
    date_time_picker, parse_local_date_time,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    date_time_picker(
        "showcase-date-time-time-options",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Configured time controls")
    .format(TimeFormat::new(
        HourCycle::H12,
        TimePrecision::Second,
        DateLocale::En,
    ))
    .time_dropdown(true)
    .time_steps(1, 15, 10)
    .reverse_time_controls(true)
    .max_time_dropdown_height(180.)
    .default_value(Some(value("2026-11-04T18:45:30")))
}
