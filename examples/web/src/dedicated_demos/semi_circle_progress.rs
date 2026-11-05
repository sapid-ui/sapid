use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(80_f64);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::semi_circle_progress(value)
                    .theme(theme.clone())
                    .name("Monthly target")
                    .size(200.)
                    .thickness(16.)
                    .label(value.map(|v| format!("{v:.0}% complete")))
                    .value_text(value.map(|v| format!("{v:.0} percent of the monthly target")))
                    .style(Style {
                        font_size: Some(18.),
                        ..Style::new()
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Advance monthly target")
                    .enabled(value.map(|v| *v < 100.))
                    .on_press(move || value.revise(|v| *v = (*v + 5.).min(100.))),
            ))
            .show_in(scope)
    })
}
