use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::card()
            .theme(theme.clone())
            .bordered(true)
            .title("Bordered card")
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("A quiet outline separates related content."),
            ),
        sapid::card()
            .theme(theme.clone())
            .bordered(false)
            .shadow(sapid::ShadowSize::Small)
            .style(Style::new().radius(16.).padding(Edges::all(20.)))
            .title("Raised card")
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("An explicit shadow and rounded surface."),
            ),
    ))
}
