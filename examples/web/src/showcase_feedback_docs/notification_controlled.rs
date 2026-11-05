use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let accept = scope.cage(false);
        let details = scope.cage(false);
        let body_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::notification("controlled-notification")
                    .theme(theme.clone())
                    .title("Controlled notification")
                    .content("This application owns dismissal and any action result.")
                    .visible(visible)
                    .close_label("Dismiss controlled notification")
                    .on_close(move || {
                        if accept.get() {
                            visible.revise(|v| *v = false)
                        }
                    })
                    .action(
                        details.map(|v| {
                            if *v {
                                "Hide notification details".to_owned()
                            } else {
                                "Show notification details".to_owned()
                            }
                        }),
                        move || details.revise(|v| *v = !*v),
                    ),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept notification dismissal")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore controlled notification")
                    .on_press(move || visible.revise(|v| *v = true)),
                when(details, move || {
                    sapid::text()
                        .theme(body_theme.clone())
                        .content("Notification details are now visible.")
                }),
            ))
            .show_in(scope)
    })
}
