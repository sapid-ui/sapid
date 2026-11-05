use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let name = scope.cage(true);
        let email = scope.cage(true);
        let density = scope.cage("comfortable".to_owned());
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept column and density changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::dropdown_menu("table-view")
                    .theme(theme.clone())
                    .label("Table view")
                    .width(240.)
                    .item(sapid::menu_label("columns", "Visible columns"))
                    .item(
                        sapid::menu_checkbox("name")
                            .label("Name column")
                            .checked(name)
                            .on_checked_change(move |next| {
                                if accept.get() {
                                    name.revise(|old| *old = next);
                                }
                            }),
                    )
                    .item(
                        sapid::menu_checkbox("email")
                            .label("Email column")
                            .checked(email)
                            .on_checked_change(move |next| {
                                if accept.get() {
                                    email.revise(|old| *old = next);
                                }
                            }),
                    )
                    .separator("density-divider")
                    .radio_group(
                        sapid::menu_radio_group("density")
                            .label("Row density")
                            .value(density)
                            .on_value_change(move |next| {
                                if accept.get() {
                                    density.revise(|old| *old = next);
                                }
                            })
                            .items([
                                sapid::menu_radio_item("comfortable", "Comfortable"),
                                sapid::menu_radio_item("compact", "Compact"),
                            ]),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        let columns: Vec<_> = [(name.get(), "Name"), (email.get(), "Email")]
                            .into_iter()
                            .filter_map(|(shown, label)| shown.then_some(label))
                            .collect();
                        format!(
                            "Columns: {}. Density: {}",
                            if columns.is_empty() {
                                "none".into()
                            } else {
                                columns.join(", ")
                            },
                            density.get()
                        )
                    })),
            ))
            .show_in(scope)
    })
}
