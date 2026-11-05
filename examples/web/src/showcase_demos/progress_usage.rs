use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(42_f64);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Upload progress: {v:.0}%"))),
                sapid::progress(value)
                    .theme(theme.clone())
                    .name("Project upload")
                    .thickness(12.)
                    .value_text(value.map(|v| format!("{v:.0} percent uploaded"))),
                sapid::group().theme(theme.clone()).gap(8.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Complete another batch")
                        .enabled(value.map(|v| *v < 100.))
                        .on_press(move || value.revise(|v| *v = (*v + 15.).min(100.))),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restart upload example")
                        .on_press(move || value.revise(|v| *v = 0.)),
                )),
            ))
            .show_in(scope)
    })
}
