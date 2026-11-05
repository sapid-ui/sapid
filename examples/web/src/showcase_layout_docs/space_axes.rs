use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::group()
            .theme(theme.clone())
            .class("docs-space-horizontal")
            .gap(0.)
            .wrap(glory::style::FlexWrap::NoWrap)
            .fill((
                sapid::badge().theme(theme.clone()).content("Before"),
                sapid::space()
                    .theme(theme.clone())
                    .width(Length::Px(32.))
                    .style(Style {
                        shrink: Some(0.),
                        ..Style::new()
                    }),
                sapid::badge().theme(theme.clone()).content("After"),
            )),
        sapid::stack()
            .theme(theme.clone())
            .class("docs-space-vertical")
            .gap(0.)
            .fill((
                sapid::text().theme(theme.clone()).content("Above"),
                sapid::space().theme(theme.clone()).height(Length::Px(24.)),
                sapid::text().theme(theme.clone()).content("Below"),
            )),
        sapid::group()
            .theme(theme.clone())
            .class("docs-space-gap")
            .gap(16.)
            .fill(Vec::from(["One", "Two", "Three"].map(|label| {
                sapid::badge().theme(theme.clone()).content(label)
            }))),
    ))
}
