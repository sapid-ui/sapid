use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::center()
            .theme(theme.clone())
            .class("docs-center-both")
            .height(Length::Px(140.))
            .style(Style {
                background: Some(theme.accent_soft),
                ..Style::new()
            })
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Horizontal and vertical"),
            ),
        sapid::center()
            .theme(theme.clone())
            .class("docs-center-horizontal")
            .height(Length::Px(100.))
            .style(Style {
                align: Some(Align::Start),
                background: Some(theme.accent_soft),
                ..Style::new()
            })
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Horizontal only"),
            ),
        sapid::box_().theme(theme.clone()).fill((
            sapid::text()
                .theme(theme.clone())
                .content("Inline content:"),
            sapid::center()
                .theme(theme.clone())
                .inline(true)
                .height(Length::Px(40.))
                .style(Style {
                    background: Some(theme.accent_soft),
                    ..Style::new().padding(Edges::all(8.))
                })
                .fill(sapid::text().theme(theme).content("A compact inline frame")),
        )),
    ))
}
