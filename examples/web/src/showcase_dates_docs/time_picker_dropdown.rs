use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, TimeFormat, TimePrecision, group, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-dropdown")
            .theme(theme.clone())
            .name("Time controls")
            .with_dropdown(true)
            .minute_step(15)
            .default_value(Some(at(10, 30, 0))),
        time_picker("showcase-time-picker-dropdown-reversed")
            .theme(theme.clone())
            .name("Reversed 12 hour controls")
            .clearable(true)
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .with_dropdown(true)
            .reverse_time_controls_list(true)
            .max_dropdown_content_height(180.)
            .default_value(Some(at(18, 45, 30))),
    ))
}
