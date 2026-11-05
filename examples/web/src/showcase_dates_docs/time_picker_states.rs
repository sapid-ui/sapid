use glory::prelude::*;
use sapid::{LocalTime, group, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-readonly")
            .theme(theme.clone())
            .name("Readonly time")
            .readonly(true)
            .clearable(true)
            .default_value(Some(at(11, 30, 0))),
        time_picker("showcase-time-picker-disabled")
            .theme(theme.clone())
            .name("Disabled time")
            .enabled(false)
            .clearable(true)
            .default_value(Some(at(13, 0, 0))),
    ))
}
