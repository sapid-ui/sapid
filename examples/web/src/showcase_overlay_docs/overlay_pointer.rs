use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let n = scope.cage(0usize);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::overlay()
                    .theme(theme.clone())
                    .color(Color(0, 0, 0, 255))
                    .opacity(0.3)
                    .fill(
                        sapid::button()
                            .theme(theme.clone())
                            .label("Activate beneath the shade")
                            .on_press(move || n.revise(|v| *v += 1)),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(n.map(|v| format!("Shaded activations: {v}"))),
            ))
            .show_in(scope)
    })
}
