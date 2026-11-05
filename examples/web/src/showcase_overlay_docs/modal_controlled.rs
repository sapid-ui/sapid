use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let allow = scope.cage(false);
        let body = theme.clone();
        sapid::dialog("modal-controlled")
            .theme(theme.clone())
            .label("Open controlled modal")
            .title("Review your draft")
            .close_label("Dismiss controlled modal")
            .open(open)
            .on_open_change(move |v| {
                if v || allow.get() {
                    open.revise(|old| *old = v)
                }
            })
            .content_policy(sapid::ContentPolicy::KeepMounted)
            .content(move || {
                sapid::stack().theme(body.clone()).gap(12.).fill((
                    sapid::text_field()
                        .theme(body.clone())
                        .name("Retained modal draft")
                        .default_value("Keep this draft"),
                    sapid::checkbox()
                        .theme(body.clone())
                        .label("Allow modal dismissal")
                        .checked(allow)
                        .on_checked_change(move |v| allow.revise(|old| *old = v)),
                ))
            })
            .show_in(scope)
    })
}
