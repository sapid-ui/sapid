use glory::prelude::*;
use sapid::{group, progress, ring_progress, semi_circle_progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    group().theme(theme.clone()).gap(24.).fill((
        progress(35.)
            .theme(theme.clone())
            .width(Length::Px(220.))
            .thickness(6.),
        ring_progress(62.)
            .theme(theme.clone())
            .size(112.)
            .thickness(12.),
        semi_circle_progress(75.)
            .theme(theme.clone())
            .size(144.)
            .thickness(10.),
    ))
}
