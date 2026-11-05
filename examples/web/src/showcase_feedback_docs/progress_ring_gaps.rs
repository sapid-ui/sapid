use glory::prelude::*;
use glory::style::Color;
use sapid::{ProgressSection, group, ring_progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    group().theme(theme.clone()).gap(28.).fill((
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .label("0° gap"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .section_gap(6.)
            .label("6° gap"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .section_gap(18.)
            .label("18° gap"),
    ))
}
