use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, TimeFormat, TimePrecision, group, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-paste-default")
            .theme(theme.clone())
            .name("Paste 17:33:43")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            )),
        time_picker("showcase-time-picker-paste-custom")
            .theme(theme.clone())
            .name("Paste noon")
            .paste_split(move |value| (value.trim() == "noon").then(|| at(12, 0, 0))),
    ))
}
