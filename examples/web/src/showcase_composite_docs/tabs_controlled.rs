use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("profile".to_owned());
        sapid::tabs("manual-settings")
            .theme(theme.clone())
            .name("Manual settings")
            .value(value)
            .activation(sapid::TabActivation::Manual)
            .on_value_change(move |next| value.revise(|old| *old = next))
            .tab(
                sapid::tab("profile")
                    .label("Profile")
                    .content(|| text().content("Use arrows to move focus, then Enter to select.")),
            )
            .tab(
                sapid::tab("security").label("Security").content(|| {
                    text().content("Manual activation is useful for expensive panels.")
                }),
            )
            .tab(
                sapid::tab("billing")
                    .label("Billing")
                    .enabled(false)
                    .content(|| text().content("Billing is unavailable.")),
            )
            .show_in(scope)
    })
}
