use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(10.).fill((
        sapid::text()
            .theme(theme.clone())
            .variant(sapid::TextVariant::Lead)
            .content("A calmer place for your next project"),
        sapid::text().theme(theme.clone()).content(TEXT_1),
        sapid::text()
            .theme(theme.clone())
            .variant(sapid::TextVariant::Muted)
            .content("Updated by the Mountain studio design team"),
    ))
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
