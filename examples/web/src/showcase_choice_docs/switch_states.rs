use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(true);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept switch changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Controlled notifications")
                    .checked(value)
                    .with_marks(true)
                    .on_checked_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v);
                        }
                    }),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Read-only notifications")
                    .default_checked(true)
                    .readonly(true),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Disabled notifications")
                    .default_checked(true)
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
