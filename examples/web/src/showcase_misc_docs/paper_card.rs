use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::paper().theme(theme.clone()).bordered(true).fill(
            sapid::text()
                .theme(theme.clone())
                .content("A simple note surface"),
        ),
        sapid::card()
            .theme(theme.clone())
            .title("A structured project card")
            .description("Header, body and footer are separate slots.")
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Use the structure when the content needs it."),
            ),
    ))
}
