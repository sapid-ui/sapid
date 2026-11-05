use glory::prelude::*;
use sapid::{LocalTime, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    time_picker("showcase-time-picker-presets")
        .theme(theme.clone())
        .name("Meeting time")
        .presets(vec![
            at(9, 0, 0),
            at(10, 30, 0),
            at(12, 0, 0),
            at(14, 30, 0),
            at(16, 0, 0),
            at(17, 30, 0),
        ])
        .default_value(Some(at(10, 30, 0)))
}
