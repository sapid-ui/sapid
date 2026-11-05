use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::paper()
        .theme(theme.clone())
        .bordered(true)
        .shadow(sapid::ShadowSize::Small)
        .style(Style::new().padding(Edges::all(20.)).radius(10.))
        .fill(
            sapid::stack().theme(theme.clone()).gap(10.).fill((
                sapid::title(3)
                    .theme(theme.clone())
                    .weight(400.)
                    .content("A place for a useful note"),
                sapid::text().theme(theme.clone()).content(TEXT_1),
            )),
        )
}

const TEXT_1: &str = "Paper supplies the surface. Your own widgets supply the content and actions.";
