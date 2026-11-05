use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().class("mark-doc-selection").fill((
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content("Review note: "),
        sapid::mark().theme(theme.clone()).content(TEXT_1),
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content(" Keep the final wording clear."),
    ))
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
