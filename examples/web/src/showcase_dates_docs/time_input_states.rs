use glory::prelude::*;
use sapid::{LocalTime, TimeBounds, group, time_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-bounded")
            .theme(theme.clone())
            .name("Office hours")
            .bounds(TimeBounds::new(Some(at(9, 0, 0)), Some(at(17, 0, 0))).unwrap())
            .default_value(Some(at(10, 0, 0))),
        time_input("showcase-time-readonly")
            .theme(theme.clone())
            .name("Readonly time")
            .readonly(true)
            .default_value(Some(at(11, 30, 0))),
        time_input("showcase-time-disabled")
            .theme(theme.clone())
            .name("Disabled time")
            .enabled(false)
            .default_value(Some(at(13, 0, 0))),
    ))
}
