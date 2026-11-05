use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, TimeFormat, TimePrecision, group, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-seconds")
            .theme(theme.clone())
            .name("Time with seconds")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_picker("showcase-time-picker-twelve")
            .theme(theme.clone())
            .name("12 hour time")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Minute,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 0))),
        time_picker("showcase-time-picker-placeholders")
            .theme(theme.clone())
            .name("Time placeholders")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_placeholder("hh")
            .minute_placeholder("mm")
            .second_placeholder("ss"),
    ))
}
