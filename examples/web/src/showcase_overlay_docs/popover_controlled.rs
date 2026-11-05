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
                    .label("Allow popover dismissal")
                    .checked(allow)
                    .on_checked_change(move |v| allow.revise(|old| *old = v)),
                sapid::popover("popover-control")
                    .theme(theme.clone())
                    .label("Open controlled popover")
                    .title("Controlled preferences")
                    .close_label("Dismiss controlled popover")
                    .open(open)
                    .dismiss_on_outside(false)
                    .on_open_change(move |v| {
                        if v || allow.get() {
                            open.revise(|old| *old = v)
                        }
                    })
                    .description("Outside clicks leave this panel open."),
            ))
            .show_in(scope)
    })
}
