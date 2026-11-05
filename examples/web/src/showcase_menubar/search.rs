use glory::prelude::*;
fn matches(label: &str, query: &str) -> bool {
    label
        .to_lowercase()
        .contains(query.trim().to_lowercase().as_str())
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let status = scope.cage("Open a file from this menu".to_owned());
        let mut files = sapid::menubar_menu("file")
            .label("Files")
            .width(280.)
            .search(
                sapid::menu_search()
                    .name("Search files")
                    .placeholder("Find a recent file")
                    .value(query)
                    .on_value_change(move |next| query.revise(|old| *old = next)),
            );
        for (key, label) in [
            ("roadmap", "Roadmap"),
            ("release", "Release notes"),
            ("handbook", "Team handbook"),
        ] {
            files = files.item(
                sapid::menu_item(key)
                    .label(label)
                    .visible(query.map(move |query| matches(label, query)))
                    .on_activate(move || status.revise(|old| *old = format!("{label} opened"))),
            );
        }
        files = files.item(
            sapid::menu_label("empty", "No matching files").visible(query.map(|query| {
                !["Roadmap", "Release notes", "Team handbook"]
                    .iter()
                    .any(|label| matches(label, query))
            })),
        );
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::menubar("searchable-bar")
                    .theme(theme.clone())
                    .name("Searchable application menu")
                    .menu(files)
                    .menu(
                        sapid::menubar_menu("help").label("Help").item(
                            sapid::menu_item("about")
                                .label("About the workspace")
                                .on_activate(move || {
                                    status
                                        .revise(|old| *old = "Workspace information opened".into())
                                }),
                        ),
                    ),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
