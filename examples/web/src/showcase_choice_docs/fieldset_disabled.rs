use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let disabled = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Disable notification settings")
                    .checked(disabled)
                    .on_checked_change(move |v| disabled.revise(|old| *old = v)),
                sapid::fieldset("notification-settings")
                    .theme(theme.clone())
                    .legend("Notification settings")
                    .disabled(disabled)
                    .fill((
                        sapid::field("notification-name")
                            .theme(theme.clone())
                            .label("Display name")
                            .control(|field| {
                                sapid::text_field().field(field).default_value("Alex")
                            }),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Send email updates"),
                        sapid::switch().theme(theme.clone()).label("Enable alerts"),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Save notification settings"),
                    )),
            ))
            .show_in(scope)
    })
}
