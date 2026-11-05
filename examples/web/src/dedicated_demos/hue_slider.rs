use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let hue = scope.cage(210.);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .style(Style::new().max_width(Limit::Px(360.)))
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(hue.map(|v| format!("Hue: {v:.0}°"))),
                sapid::hue_slider("accent-hue")
                    .theme(theme.clone())
                    .name("Hue angle")
                    .value(hue)
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| hue.revise(|old| *old = v)),
                sapid::color_swatch(scope.bond(move || {
                    sapid::Hsva {
                        hue: hue.get(),
                        ..Default::default()
                    }
                    .to_rgba()
                    .expect("bounded hue")
                }))
                .theme(theme.clone())
                .name("Selected hue preview")
                .side(40.),
            ))
            .show_in(scope)
    })
}
