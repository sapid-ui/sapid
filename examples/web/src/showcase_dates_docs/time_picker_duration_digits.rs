use glory::prelude::*;
use sapid::{TimeDuration, group, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-duration-two-digits")
            .theme(theme.clone())
            .name("Two minimum hour digits")
            .duration()
            .min_hours_digits(2)
            .default_value(Some(TimeDuration::new(7, 5, 0).unwrap())),
        time_picker("showcase-duration-four-digits")
            .theme(theme.clone())
            .name("Four minimum hour digits")
            .duration()
            .min_hours_digits(4)
            .default_value(Some(TimeDuration::new(7, 5, 0).unwrap())),
    ))
}
