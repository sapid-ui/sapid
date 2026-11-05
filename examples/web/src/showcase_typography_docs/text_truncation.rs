use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::text()
            .theme(theme.clone())
            .class("text-doc-truncate")
            .truncate(true)
            .style(
                Style::new()
                    .width(Length::Px(220.))
                    .max_width(glory::style::Limit::Percent(100.)),
            )
            .content(TEXT_1),
        sapid::text()
            .theme(theme.clone())
            .class("text-doc-clamp")
            .line_clamp(2)
            .style(
                Style::new()
                    .width(Length::Px(260.))
                    .max_width(glory::style::Limit::Percent(100.)),
            )
            .content(TEXT_2),
    ))
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
const TEXT_2: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
