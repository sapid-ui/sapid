use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::text_list()
        .theme(theme.clone())
        .name("Spacious project notes")
        .item(
            sapid::text_list_item()
                .style(Style::new().margin(Edges {
                    top: 0.,
                    right: 0.,
                    bottom: 12.,
                    left: 0.,
                }))
                .fill(sapid::text().theme(theme.clone()).content(TEXT_1)),
        )
        .item(
            sapid::text_list_item().fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Keep the final action short, clear and easy to discover."),
            ),
        )
}

const TEXT_1: &str = "A shared design system gives a team clear patterns for writing, reviewing and delivering useful work. Thoughtful defaults leave room for the details that make each project different.";
