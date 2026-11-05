use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let status = scope.cage("Untitled document".to_owned());
        let ruler = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::menubar("document-menu")
                    .theme(theme.clone())
                    .name("Document menu")
                    .menus([
                        sapid::menubar_menu("file")
                            .label("File")
                            .width(240.)
                            .item(sapid::menu_label("document", "Document"))
                            .item(
                                sapid::menu_item("new")
                                    .label("New document")
                                    .right_section(glory::ui::text().content("Ctrl+N"))
                                    .on_activate(move || {
                                        status.revise(|old| *old = "New untitled document".into())
                                    }),
                            )
                            .submenu(
                                sapid::menu_submenu("recent")
                                    .label("Open recent")
                                    .width(200.)
                                    .item(sapid::menu_item("roadmap").label("Roadmap").on_activate(
                                        move || status.revise(|old| *old = "Roadmap opened".into()),
                                    ))
                                    .item(
                                        sapid::menu_item("release")
                                            .label("Release notes")
                                            .on_activate(move || {
                                                status.revise(|old| {
                                                    *old = "Release notes opened".into()
                                                })
                                            }),
                                    ),
                            )
                            .separator("save-actions")
                            .item(
                                sapid::menu_item("save")
                                    .label("Save document")
                                    .right_section(glory::ui::text().content("Ctrl+S"))
                                    .on_activate(move || {
                                        status.revise(|old| *old = "Document saved".into())
                                    }),
                            ),
                        sapid::menubar_menu("view")
                            .label("View")
                            .width(220.)
                            .item(
                                sapid::menu_checkbox("ruler")
                                    .label("Show ruler")
                                    .checked(ruler)
                                    .on_checked_change(move |next| ruler.revise(|old| *old = next)),
                            )
                            .item(sapid::menu_item("zoom").label("Reset zoom").on_activate(
                                move || status.revise(|old| *old = "Zoom reset to 100%".into()),
                            )),
                        sapid::menubar_menu("help").label("Help").item(
                            sapid::menu_item("about")
                                .label("About this editor")
                                .on_activate(move || {
                                    status.revise(|old| *old = "Sapid document editor".into())
                                }),
                        ),
                    ]),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
