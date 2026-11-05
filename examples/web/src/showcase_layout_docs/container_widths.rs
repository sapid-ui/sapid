use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let content = |label| {
        sapid::box_()
            .theme(theme.clone())
            .style(Style {
                background: Some(theme.accent_soft),
                ..Style::new().padding(Edges::all(12.))
            })
            .fill(sapid::text().theme(theme.clone()).content(label))
    };
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::container()
            .theme(theme.clone())
            .class("docs-container-small")
            .max_width(Limit::Px(240.))
            .fill(content("Maximum 240px")),
        sapid::container()
            .theme(theme.clone())
            .class("docs-container-large")
            .max_width(Limit::Px(420.))
            .fill(content("Maximum 420px")),
        sapid::container()
            .theme(theme.clone())
            .class("docs-container-fluid")
            .fluid()
            .fill(content("Fluid: all available content width")),
        sapid::container().theme(theme.clone()).fluid().fill(
            sapid::container()
                .theme(theme.clone())
                .fluid()
                .padding(Edges::all(0.))
                .fill(content(
                    "Nested container with its inner padding explicitly removed",
                )),
        ),
    ))
}
