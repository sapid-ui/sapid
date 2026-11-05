use glory::prelude::*;
use glory::style::Color;
use sapid::{SemiCircleFillDirection, SemiCircleOrientation, group, semi_circle_progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    group().theme(theme.clone()).gap(28.).fill((
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .label("Up · left to right")
            .filled_segment_color(Color(240, 62, 62, 255))
            .empty_segment_color(Color(64, 192, 87, 255)),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .fill_direction(SemiCircleFillDirection::RightToLeft)
            .label("Up · right to left"),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .semi_orientation(SemiCircleOrientation::Down)
            .label("Down · left to right"),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .semi_orientation(SemiCircleOrientation::Down)
            .fill_direction(SemiCircleFillDirection::RightToLeft)
            .label("Down · right to left"),
    ))
}
