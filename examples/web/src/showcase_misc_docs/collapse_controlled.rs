use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let allow = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow delivery details")
                    .checked(allow)
                    .on_checked_change(move |v| allow.revise(|s| *s = v)),
                sapid::collapsible("controlled-delivery")
                    .theme(theme.clone())
                    .label("Review delivery details")
                    .open(open)
                    .on_open_change(move |v| {
                        if allow.get() {
                            open.revise(|s| *s = v)
                        }
                    })
                    .height_transition(180)
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::text().theme(theme.clone()).content(
                                "Packages can be left with the reception desk on weekdays.",
                            )
                        }
                    }),
            ))
            .show_in(scope)
    })
}
