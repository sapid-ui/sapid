use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        sapid::group()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::burger()
                    .theme(theme.clone())
                    .open(open)
                    .on_open_change(move |next| open.revise(|value| *value = next)),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        if open.get() {
                            "Navigation is open"
                        } else {
                            "Navigation is closed"
                        }
                        .to_owned()
                    })),
            ))
            .show_in(scope)
    })
}
