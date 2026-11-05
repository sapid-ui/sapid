use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::box_()
        .theme(theme.clone())
        .style(Style {
            background: Some(theme.accent_soft),
            ..Style::new().padding(Edges::all(20.)).radius(8.)
        })
        .fill(
            sapid::stack().theme(theme.clone()).gap(10.).fill((
                sapid::title(3)
                    .theme(theme.clone())
                    .weight(400.)
                    .content("Your next project"),
                sapid::text().theme(theme.clone()).content(TEXT_1),
            )),
        )
}

const TEXT_1: &str = "Keep the brief, notes and next steps together in a simple shared space.";
