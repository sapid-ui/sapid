use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::time_value(Some(clock("18:45")))
            .theme(theme.clone())
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Minute,
                DateLocale::En,
            )),
        sapid::time_value(Some(clock("18:45")))
            .theme(theme.clone())
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Minute,
                DateLocale::En,
            )),
        sapid::time_value(Some(clock("18:45:30")))
            .theme(theme.clone())
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::ZhCn,
            )),
    ))
}
fn clock(value: &str) -> LocalTime {
    LocalTime::parse_iso(value).unwrap()
}
