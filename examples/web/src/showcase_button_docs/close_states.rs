use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shown = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(12.).fill((
                    sapid::text().theme(theme.clone()).content(shown.map(|v| {
                        if *v {
                            "Announcement visible"
                        } else {
                            "Announcement dismissed"
                        }
                        .to_owned()
                    })),
                    sapid::close_button()
                        .theme(theme.clone())
                        .name("Dismiss announcement")
                        .enabled(shown)
                        .on_press(move || shown.revise(|v| *v = false)),
                )),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore announcement")
                    .on_press(move || shown.revise(|v| *v = true)),
                sapid::close_button()
                    .theme(theme.clone())
                    .name("Unavailable close")
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
