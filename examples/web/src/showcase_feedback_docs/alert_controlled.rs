use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let accept = scope.cage(false);
        let requests = scope.cage(0_u32);
        let label = scope.cage("Dismiss controlled notice".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::alert("controlled-notice")
                    .theme(theme.clone())
                    .title("Controlled notice")
                    .content("Closing proposes a change. This application can refuse it.")
                    .announcement(sapid::Announcement::Off)
                    .visible(visible)
                    .closable(true)
                    .close_label(label)
                    .on_close(move || {
                        requests.revise(|v| *v += 1);
                        if accept.get() {
                            visible.revise(|v| *v = false)
                        }
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept notice dismissal")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Notice close label")
                    .value(label)
                    .on_value_change(move |v| {
                        if !v.trim().is_empty() && v.len() <= 64 {
                            label.revise(|old| *old = v)
                        }
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore controlled notice")
                    .on_press(move || visible.revise(|v| *v = true)),
                sapid::text()
                    .theme(theme.clone())
                    .content(requests.map(|v| format!("Dismissal requests: {v}"))),
            ))
            .show_in(scope)
    })
}
