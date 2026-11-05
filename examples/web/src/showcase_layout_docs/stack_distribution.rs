use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let spread = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Distribute through the available height")
                    .checked(spread)
                    .on_checked_change(move |v| spread.revise(|old| *old = v)),
                sapid::stack()
                    .theme(theme.clone())
                    .class("docs-stack-distribution")
                    .gap(12.)
                    .style(spread.map(|v| Style {
                        align: Some(if *v { Align::Start } else { Align::Stretch }),
                        justify: Some(if *v { Justify::Between } else { Justify::Start }),
                        ..Style::new().height(Length::Px(240.))
                    }))
                    .fill(Vec::from(
                        [
                            "Plan",
                            "A longer task title can wrap within its available width",
                            "Deliver",
                        ]
                        .map(|label| {
                            sapid::text()
                                .theme(theme.clone())
                                .style(Style {
                                    background: Some(theme.accent_soft),
                                    ..Style::new()
                                        .padding(Edges::all(8.))
                                        .max_width(Limit::Percent(100.))
                                })
                                .content(label)
                        }),
                    )),
            ))
            .show_in(scope)
    })
}
