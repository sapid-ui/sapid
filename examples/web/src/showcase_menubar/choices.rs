use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let sidebar = scope.cage(true);
        let status = scope.cage(false);
        let density = scope.cage("comfortable".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::menubar("docs-choices")
                    .theme(theme.clone())
                    .name("View preferences")
                    .menu(
                        sapid::menubar_menu("view")
                            .label("View")
                            .item(
                                sapid::menu_checkbox("sidebar")
                                    .label("Show sidebar")
                                    .checked(sidebar)
                                    .on_checked_change(move |next| {
                                        sidebar.revise(|value| *value = next)
                                    }),
                            )
                            .item(
                                sapid::menu_checkbox("status")
                                    .label("Show status bar")
                                    .checked(status)
                                    .on_checked_change(move |next| {
                                        status.revise(|value| *value = next)
                                    }),
                            )
                            .separator("appearance")
                            .radio_group(
                                sapid::menu_radio_group("density")
                                    .label("Appearance")
                                    .value(density)
                                    .on_value_change(move |next| {
                                        density.revise(|value| *value = next)
                                    })
                                    .items([
                                        sapid::menu_radio_item("compact", "Compact"),
                                        sapid::menu_radio_item("comfortable", "Comfortable"),
                                        sapid::menu_radio_item("spacious", "Spacious"),
                                    ]),
                            ),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        format!(
                            "Sidebar: {} · Status bar: {} · Density: {}",
                            if sidebar.get() { "shown" } else { "hidden" },
                            if status.get() { "shown" } else { "hidden" },
                            density.get()
                        )
                    })),
            ))
            .show_in(scope)
    })
}
