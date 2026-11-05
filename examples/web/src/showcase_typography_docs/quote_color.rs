use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::blockquote()
        .theme(theme.clone())
        .style(Style {
            border_color: Some(theme.muted),
            background: Some(theme.surface),
            ..Style::new()
        })
        .fill(sapid::text().theme(theme.clone()).content(TEXT_1))
        .citation(
            sapid::text()
                .theme(theme.clone())
                .variant(sapid::TextVariant::Small)
                .content("— Design system handbook, chapter two"),
        )
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
