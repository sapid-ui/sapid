use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::group()
            .theme(theme.clone())
            .class("docs-group-equal")
            .wrap(glory::style::FlexWrap::NoWrap)
            .gap(8.)
            .fill(Vec::from(["Save", "Review", "Publish"].map(|label| {
                sapid::button()
                    .theme(theme.clone())
                    .label(label)
                    .style(Style {
                        grow: Some(1.),
                        shrink: Some(1.),
                        ..Style::new().width(Length::Px(0.)).min_width(Length::Px(0.))
                    })
            }))),
        sapid::group()
            .theme(theme.clone())
            .class("docs-group-long")
            .wrap(glory::style::FlexWrap::NoWrap)
            .gap(12.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(
                        "A long document title wraps in the flexible space beside a stable action.",
                    )
                    .style(Style {
                        grow: Some(1.),
                        shrink: Some(1.),
                        ..Style::new().min_width(Length::Px(0.))
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Open")
                    .style(Style {
                        shrink: Some(0.),
                        ..Style::new()
                    }),
            )),
    ))
}
