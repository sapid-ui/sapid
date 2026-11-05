use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("outer-reading")
        .theme(theme.clone())
        .name("Outer reading notes")
        .overscroll_behavior(sapid::OverscrollBehavior::Contain)
        .style(
            Style::new()
                .height(Length::Px(280.))
                .max_width(glory::style::Limit::Percent(100.)),
        )
        .content(move |_| {
            let inner = theme.clone();
            sapid::stack().theme(theme.clone()).gap(18.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .content("Project overview and review notes"),
                sapid::scroll_area("inner-reading")
                    .theme(theme.clone())
                    .name("Inner reading notes")
                    .scrollbars(sapid::Scrollbars::Always)
                    .offset_scrollbars(sapid::ScrollbarOffset::Both)
                    .scrollbar_size(10.)
                    .overscroll_behavior(sapid::OverscrollBehavior::Contain)
                    .style(Style::new().height(Length::Px(140.)))
                    .content(move |_| {
                        sapid::text()
                            .theme(inner.clone())
                            .style(Style::new().min_height(Length::Px(500.)))
                            .content("A long inner reading note stays in its own scroll region.")
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .style(Style::new().min_height(Length::Px(400.)))
                    .content("Continue reading the outer project summary."),
            ))
        })
}
