use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva {
            hue: 208.,
            saturation: 0.85,
            value: 0.9,
            alpha: 0.7,
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .style(Style::new().max_width(Limit::Px(320.)))
            .fill((
                sapid::color_picker("accent-picker")
                    .theme(theme.clone())
                    .name("Accent color picker")
                    .value(color)
                    .full_width(true)
                    .on_value_change(move |v| color.revise(|old| *old = v)),
                sapid::text().theme(theme.clone()).content(color.map(|v| {
                    format!(
                        "Selected: {}",
                        sapid::format_color(
                            v.to_rgba().expect("bounded picker color"),
                            sapid::ColorFormat::Hexa
                        )
                    )
                })),
            ))
            .show_in(scope)
    })
}
