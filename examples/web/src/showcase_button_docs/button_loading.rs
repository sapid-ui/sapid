use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let count = scope.cage(0);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Save changes")
                    .loading(loading)
                    .on_press(move || count.revise(|v| *v += 1)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Saving changes")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|v| format!("Save requests: {v}"))),
                sapid::button()
                    .theme(theme.clone())
                    .label("Unavailable save")
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
