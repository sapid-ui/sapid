use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        glory::ui::view().fill(
            sapid::highlight()
                .theme(theme.clone())
                .content("Nothing in this paragraph matches the current query.")
                .query("missing"),
        ),
        glory::ui::view().fill(
            sapid::highlight()
                .theme(theme.clone())
                .content(TEXT_1)
                .queries(["team".to_owned(), "project".to_owned()]),
        ),
    ))
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
