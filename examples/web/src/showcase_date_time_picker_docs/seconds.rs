use glory::prelude::*;
use sapid::{
    DateLocale, DateTimeDisplayFormat, DateTimePolicy, DatesProvider, HourCycle, TimeFormat,
    TimePrecision, date_time_picker, parse_local_date_time,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    date_time_picker(
        "showcase-date-time-seconds",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Date and time with seconds")
    .format(TimeFormat::new(
        HourCycle::H12,
        TimePrecision::Second,
        DateLocale::En,
    ))
    .display_format(DateTimeDisplayFormat::DayMonthYear)
    .placeholder("Choose date and time")
    .default_value(Some(value("2026-11-04T18:45:30")))
}
