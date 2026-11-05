use glory::prelude::*;
use sapid::{LocalTime, group, time_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-placeholder")
            .theme(theme.clone())
            .name("Empty time")
            .placeholder("HH:MM"),
        time_input("showcase-time-prefilled")
            .theme(theme.clone())
            .name("Prefilled time")
            .placeholder("HH:MM")
            .default_value(Some(at(9, 15, 0))),
    ))
}
