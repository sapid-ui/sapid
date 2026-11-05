use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::text()
            .theme(theme.clone())
            .content("The same transparency over two different surfaces"),
        sapid::box_()
            .theme(theme.clone())
            .style(Style {
                background: Some(Color(255, 255, 255, 255)),
                ..Style::new().padding(Edges::all(16.))
            })
            .fill(
                sapid::alpha_slider("opacity-light-surface")
                    .theme(theme.clone())
                    .name("Opacity over a light surface")
                    .color(theme.accent)
                    .default_value(0.3)
                    .style(Style::new().width(Length::Percent(100.))),
            ),
        sapid::box_()
            .theme(theme.clone())
            .style(Style {
                background: Some(Color(30, 30, 30, 255)),
                ..Style::new().padding(Edges::all(16.))
            })
            .fill(
                sapid::alpha_slider("opacity-dark-surface")
                    .theme(theme.clone())
                    .name("Opacity over a dark surface")
                    .color(theme.accent)
                    .default_value(0.7)
                    .style(Style::new().width(Length::Percent(100.))),
            ),
        sapid::alpha_slider("opacity-readonly")
            .theme(theme.clone())
            .name("Read-only opacity")
            .default_value(0.4)
            .readonly(true),
        sapid::alpha_slider("opacity-disabled")
            .theme(theme)
            .name("Disabled opacity")
            .default_value(0.6)
            .enabled(false),
    ))
}
