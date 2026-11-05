use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let email = scope.cage(true);
        let push = scope.cage(false);
        let locked = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Disable notification channels")
                    .checked(locked)
                    .on_checked_change(move |v| locked.revise(|old| *old = v)),
                sapid::fieldset("switch-channels")
                    .theme(theme.clone())
                    .legend("Notification channels")
                    .disabled(locked)
                    .fill((
                        sapid::switch()
                            .theme(theme.clone())
                            .label("Email notifications")
                            .checked(email)
                            .enabled(locked.map(|v| !*v))
                            .on_checked_change(move |v| email.revise(|old| *old = v)),
                        sapid::switch()
                            .theme(theme.clone())
                            .label("Push notifications")
                            .checked(push)
                            .enabled(locked.map(|v| !*v))
                            .on_checked_change(move |v| push.revise(|old| *old = v)),
                    )),
                sapid::text().theme(theme.clone()).content(
                    scope.bond(move || format!("Email: {}, push: {}", email.get(), push.get())),
                ),
            ))
            .show_in(scope)
    })
}
