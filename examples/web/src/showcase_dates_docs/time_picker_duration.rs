use glory::prelude::*;
use sapid::{TimeDuration, TimePrecision, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    time_picker("showcase-time-picker-duration")
        .theme(theme.clone())
        .name("Elapsed duration")
        .duration()
        .precision(TimePrecision::Second)
        .clearable(true)
        .default_value(Some(TimeDuration::new(125, 30, 45).unwrap()))
}
