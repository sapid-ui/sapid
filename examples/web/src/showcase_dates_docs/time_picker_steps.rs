use glory::prelude::*;
use sapid::{LocalTime, TimeBounds, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    time_picker("showcase-time-picker-steps")
        .theme(theme.clone())
        .name("Quarter hour appointment")
        .hour_step(2)
        .minute_step(15)
        .bounds(TimeBounds::new(Some(at(8, 0, 0)), Some(at(18, 0, 0))).unwrap())
        .default_value(Some(at(10, 30, 0)))
}
