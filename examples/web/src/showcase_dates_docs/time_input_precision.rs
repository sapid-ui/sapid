use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, TimeFormat, TimePrecision, group, time_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-24h")
            .theme(theme.clone())
            .name("24 hour minutes")
            .default_value(Some(at(18, 45, 0))),
        time_input("showcase-time-seconds")
            .theme(theme.clone())
            .name("24 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_input("showcase-time-12h")
            .theme(theme.clone())
            .name("12 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_input("showcase-time-12h-zh")
            .theme(theme.clone())
            .name("Chinese 12 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::ZhCn,
            ))
            .default_value(Some(at(18, 45, 30))),
    ))
}
