use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let allow = scope.cage(false);
        let body = theme.clone();
        sapid::drawer("drawer-controlled")
            .theme(theme.clone())
            .label("Open controlled drawer")
            .title("Controlled workspace")
            .close_label("Dismiss controlled drawer")
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
                        .name("Retained drawer draft")
                        .default_value("Keep this draft"),
                    sapid::checkbox()
                        .theme(body.clone())
                        .label("Allow drawer dismissal")
                        .checked(allow)
                        .on_checked_change(move |v| allow.revise(|old| *old = v)),
                ))
            })
            .show_in(scope)
    })
}
