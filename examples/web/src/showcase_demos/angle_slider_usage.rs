use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let angle = scope.cage(45.);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .align(Align::Start)
            .fill((
                sapid::angle_slider("rotation-angle")
                    .theme(theme.clone())
                    .name("Rotation angle")
                    .value(angle)
                    .on_value_change(move |v| angle.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(angle.map(|v| format!("Rotation: {v:.0}°"))),
            ))
            .show_in(scope)
    })
}
