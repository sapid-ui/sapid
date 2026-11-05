use glory::prelude::*;
use glory::style::Color;
use sapid::{ProgressSection, group, ring_progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    group().theme(theme.clone()).gap(24.).fill((
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(0.)
            .root_color(Color(222, 226, 230, 255))
            .label("0°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(90.)
            .label("90°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(180.)
            .label("180°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(270.)
            .label("270°"),
    ))
}
