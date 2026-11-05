use glory::prelude::*;
use sapid::{LocalTime, TimeGridSpec, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    time_picker("showcase-time-picker-grid")
        .theme(theme.clone())
        .name("Available time slots")
        .grid(TimeGridSpec::new(at(9, 0, 0), at(12, 0, 0), 1800, 4).unwrap())
        .disable_time(at(10, 30, 0))
        .default_value(Some(at(9, 30, 0)))
}
