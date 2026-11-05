use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let weekly = scope.cage(true);
        let saved = scope.cage(false);
        let body_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::popover("notification-preferences")
                    .theme(theme.clone())
                    .label("Notification preferences")
                    .title("Notification preferences")
                    .width(sapid::PopoverWidth::Px(300.))
                    .with_arrow(true)
                    .close_label("Close notification preferences")
                    .open(open)
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .content(move || {
                        sapid::stack().theme(body_theme.clone()).gap(12.).fill((
                            sapid::checkbox()
                                .theme(body_theme.clone())
                                .label("Weekly summary emails")
                                .checked(weekly)
                                .on_checked_change(move |v| weekly.revise(|old| *old = v)),
                            sapid::button()
                                .theme(body_theme.clone())
                                .label("Apply notification preferences")
                                .on_press(move || {
                                    saved.revise(|v| *v = true);
                                    open.revise(|v| *v = false);
                                }),
                        ))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        if saved.get() {
                            format!(
                                "Weekly summary: {}",
                                if weekly.get() { "enabled" } else { "disabled" }
                            )
                        } else {
                            "Choose how often your workspace sends email.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
