use glory::prelude::*;
use glory::style::Color;
use sapid::{ProgressOrientation, ProgressSection, group, progress};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    group().theme(theme.clone()).gap(36.).fill((
        progress(68.)
            .theme(theme.clone())
            .name("Vertical upload")
            .orientation(ProgressOrientation::Vertical)
            .length(Length::Px(180.))
            .thickness(12.),
        progress(60.)
            .theme(theme.clone())
            .name("Vertical storage by category")
            .vertical(true)
            .length(Length::Px(180.))
            .thickness(18.)
            .sections(sections),
        progress(40.)
            .theme(theme.clone())
            .name("Vertical loading")
            .vertical(true)
            .length(Length::Px(180.))
            .thickness(12.)
            .indeterminate(true),
    ))
}
