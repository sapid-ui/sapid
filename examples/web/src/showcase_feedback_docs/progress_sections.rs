use glory::prelude::*;
use glory::style::Color;
use sapid::{ProgressSection, progress, ring_progress, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    stack().theme(theme.clone()).gap(24.).fill((
        progress(60.)
            .theme(theme.clone())
            .name("Storage by category")
            .width(Length::Px(320.))
            .thickness(26.)
            .section_label(0, "Docs")
            .section_label(1, "Photos")
            .section_label(2, "Other")
            .value_text("Documents 20, photos 30 and other files 10 percent")
            .sections(sections),
        ring_progress(60.)
            .theme(theme.clone())
            .name("Work by category")
            .label("60%")
            .size(132.)
            .thickness(14.)
            .sections(sections),
    ))
}
