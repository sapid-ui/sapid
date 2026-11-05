use glory::prelude::*;
fn matches(label: &str, query: &str) -> bool {
    label
        .to_lowercase()
        .contains(query.trim().to_lowercase().as_str())
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let persistent = scope.cage(String::new());
        let options = scope.cage(String::new());
        let ruler = scope.cage(false);
        let density = scope.cage("comfortable".to_owned());
        let status = scope.cage("Choose a workspace destination".to_owned());
        let mut menu = sapid::dropdown_menu("search-destinations")
            .theme(theme.clone())
            .label("Find a destination")
            .width(280.)
            .search(
                sapid::menu_search()
                    .name("Search destinations")
                    .value(query)
                    .on_value_change(move |next| query.revise(|old| *old = next)),
            )
            .item(sapid::menu_label("results", "Workspace destinations"));
        for (key, label) in [
            ("dashboard", "Dashboard"),
            ("orders", "Orders"),
            ("reports", "Reports"),
        ] {
            menu = menu.item(
                sapid::menu_item(key)
                    .label(label)
                    .visible(query.map(move |query| matches(label, query)))
                    .on_activate(move || status.revise(|old| *old = format!("{label} selected"))),
            );
        }
        menu = menu
            .submenu(
                sapid::menu_submenu("settings")
                    .label("Settings")
                    .width(220.)
                    .visible(query.map(|query| {
                        ["Settings", "Profile", "Billing"]
                            .iter()
                            .any(|label| matches(label, query))
                    }))
                    .item(
                        sapid::menu_item("profile")
                            .label("Profile")
                            .visible(query.map(|query| {
                                matches("Settings", query) || matches("Profile", query)
                            }))
                            .on_activate(move || {
                                status.revise(|old| *old = "Profile selected".into())
                            }),
                    )
                    .item(
                        sapid::menu_item("billing")
                            .label("Billing")
                            .visible(query.map(|query| {
                                matches("Settings", query) || matches("Billing", query)
                            }))
                            .on_activate(move || {
                                status.revise(|old| *old = "Billing selected".into())
                            }),
                    ),
            )
            .item(
                sapid::menu_label("empty", "Nothing found").visible(query.map(|query| {
                    ![
                        "Dashboard",
                        "Orders",
                        "Reports",
                        "Settings",
                        "Profile",
                        "Billing",
                    ]
                    .iter()
                    .any(|label| matches(label, query))
                })),
            );
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                menu,
                sapid::dropdown_menu("persistent-search")
                    .theme(theme.clone())
                    .label("Preserve a search query")
                    .width(260.)
                    .search(
                        sapid::menu_search()
                            .name("Persistent menu search")
                            .value(persistent)
                            .clear_on_close(false)
                            .on_value_change(move |next| persistent.revise(|old| *old = next)),
                    )
                    .item(
                        sapid::menu_item("projects")
                            .label("Projects")
                            .visible(persistent.map(|query| matches("Projects", query))),
                    )
                    .item(
                        sapid::menu_item("team")
                            .label("Team")
                            .visible(persistent.map(|query| matches("Team", query))),
                    ),
                sapid::dropdown_menu("search-display-options")
                    .theme(theme.clone())
                    .label("Find a display option")
                    .width(280.)
                    .search(
                        sapid::menu_search()
                            .name("Search display options")
                            .value(options)
                            .on_value_change(move |next| options.revise(|old| *old = next)),
                    )
                    .item(
                        sapid::menu_checkbox("ruler")
                            .label("Show ruler")
                            .checked(ruler)
                            .visible(options.map(|query| matches("Show ruler", query)))
                            .on_checked_change(move |next| ruler.revise(|old| *old = next)),
                    )
                    .radio_group(
                        sapid::menu_radio_group("density")
                            .label("Page density")
                            .value(density)
                            .on_value_change(move |next| density.revise(|old| *old = next))
                            .items([
                                sapid::menu_radio_item("comfortable", "Comfortable")
                                    .visible(options.map(|query| matches("Comfortable", query))),
                                sapid::menu_radio_item("compact", "Compact")
                                    .visible(options.map(|query| matches("Compact", query))),
                            ]),
                    ),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
