use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let alpha = scope.cage(0.65);
        let color = theme.accent;
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .style(Style::new().max_width(Limit::Px(360.)))
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(alpha.map(|v| format!("Opacity: {:.0}%", v * 100.))),
                sapid::alpha_slider("accent-opacity")
                    .theme(theme.clone())
                    .name("Accent opacity")
                    .color(color)
                    .value(alpha)
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| alpha.revise(|old| *old = v)),
                sapid::color_swatch(scope.bond(move || {
                    let Color(r, g, b, _) = color;
                    Color(r, g, b, (alpha.get() * 255.).round() as u8)
                }))
                .theme(theme.clone())
                .name("Selected opacity preview")
                .side(40.),
            ))
            .show_in(scope)
    })
}
