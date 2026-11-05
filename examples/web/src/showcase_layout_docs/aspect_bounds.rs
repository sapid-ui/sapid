use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(16.).wrap(glory::style::FlexWrap::NoWrap).align(Align::Center).fill((
        sapid::aspect_ratio(1.).theme(theme.clone()).width(Length::Px(96.))
            .style(Style {shrink:Some(0.),background:Some(theme.accent_soft),..Style::new()})
            .fill(sapid::text().theme(theme.clone()).content("1 : 1")),
        sapid::text().theme(theme).style(Style::new().min_width(Length::Px(0.)))
            .content("An explicit width keeps this thumbnail square in a flex row. Long content wraps beside it; it does not set the height of the ratio frame."),
    ))
}
