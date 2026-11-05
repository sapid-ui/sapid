use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .gap(14.)
        .wrap(glory::style::FlexWrap::NoWrap)
        .style(Style::new().height(Length::Px(48.)))
        .fill((
            sapid::text().theme(theme.clone()).content("Projects"),
            sapid::separator()
                .theme(theme.clone())
                .orientation(glory::render::Orientation::Vertical)
                .length(Length::Px(32.))
                .name("Projects and settings boundary"),
            sapid::text().theme(theme.clone()).content("Settings"),
        ))
}
