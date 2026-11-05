use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::pill()
                    .theme(theme.clone())
                    .content("Design")
                    .visible(visible)
                    .removable("Remove Design topic")
                    .on_remove(move || {
                        if accept.get() {
                            visible.revise(|v| *v = false)
                        }
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept topic removal")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore Design topic")
                    .on_press(move || visible.revise(|v| *v = true)),
            ))
            .show_in(scope)
    })
}
