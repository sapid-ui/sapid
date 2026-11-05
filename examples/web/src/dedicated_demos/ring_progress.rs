use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(64_f64);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::ring_progress(value)
                    .theme(theme.clone())
                    .name("Storage usage")
                    .size(160.)
                    .thickness(12.)
                    .label(value.map(|v| format!("{v:.0}%")))
                    .value_text(value.map(|v| format!("{v:.0} percent of storage used")))
                    .style(
                        Style {
                            font_size: Some(22.),
                            ..Style::new()
                        }
                        .font_weight(500.),
                    ),
                sapid::button()
                    .theme(theme.clone())
                    .label("Use ten percent more storage")
                    .enabled(value.map(|v| *v < 100.))
                    .on_press(move || value.revise(|v| *v = (*v + 10.).min(100.))),
            ))
            .show_in(scope)
    })
}
