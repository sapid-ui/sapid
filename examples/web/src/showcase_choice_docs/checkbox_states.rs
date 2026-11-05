use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(true);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept checkbox changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Controlled subscription")
                    .checked(value)
                    .on_checked_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v);
                        }
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Read-only subscription")
                    .default_checked(true)
                    .readonly(true),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Disabled subscription")
                    .default_checked(true)
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
