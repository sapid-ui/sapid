use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selection = scope.cage("No recent project selected".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::menubar("docs-submenus")
                    .theme(theme.clone())
                    .name("Recent projects menu")
                    .menu(
                        sapid::menubar_menu("file")
                            .label("File")
                            .item(sapid::menu_item("new").label("New file"))
                            .submenu(
                                sapid::menu_submenu("recent")
                                    .label("Open recent")
                                    .item(
                                        sapid::menu_item("alpha")
                                            .label("project-alpha")
                                            .on_activate(move || {
                                                selection.revise(|value| {
                                                    *value = "Opened project-alpha".into()
                                                })
                                            }),
                                    )
                                    .item(
                                        sapid::menu_item("beta").label("project-beta").on_activate(
                                            move || {
                                                selection.revise(|value| {
                                                    *value = "Opened project-beta".into()
                                                })
                                            },
                                        ),
                                    ),
                            ),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || selection.get())),
            ))
            .show_in(scope)
    })
}
