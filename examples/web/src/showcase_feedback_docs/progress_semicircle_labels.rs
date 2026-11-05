use glory::prelude::*;
use sapid::{SemiCircleLabelPosition, group, semi_circle_progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    group().theme(theme.clone()).gap(36.).fill((
        semi_circle_progress(30.)
            .theme(theme.clone())
            .size(180.)
            .label("Bottom"),
        semi_circle_progress(30.)
            .theme(theme.clone())
            .size(180.)
            .label("Center")
            .label_position(SemiCircleLabelPosition::Center),
    ))
}
