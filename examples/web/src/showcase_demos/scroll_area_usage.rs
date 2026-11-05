use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("studio-reading")
        .theme(theme.clone())
        .name("Studio reading region")
        .style(
            Style::new()
                .height(Length::Px(240.))
                .max_width(glory::style::Limit::Percent(100.)),
        )
        .content(move |_| {
            sapid::stack().theme(theme.clone()).gap(14.).fill((
                sapid::title(4)
                    .theme(theme.clone())
                    .weight(400.)
                    .content("Studio reading notes"),
                sapid::stack().theme(theme.clone()).gap(16.).fill(
                    (1..=18)
                        .map(|n| sapid::text().theme(theme.clone()).content(format!("Note {n}: Keep project decisions clear, review the details together and make the next action easy to understand.")))
                        .collect::<Vec<_>>(),
                ),
            ))
        })
}
