use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(None::<String>);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Secondary)
                        .label("Open File")
                        .on_press(move || open.revise(|value| *value = Some("file".into()))),
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Secondary)
                        .label("Open Edit")
                        .on_press(move || open.revise(|value| *value = Some("edit".into()))),
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Secondary)
                        .label("Close all")
                        .on_press(move || open.revise(|value| *value = None)),
                )),
                sapid::menubar("docs-controlled")
                    .theme(theme.clone())
                    .name("Controlled menu")
                    .value(open)
                    .on_value_change(move |next| open.revise(|value| *value = next))
                    .menus([
                        sapid::menubar_menu("file")
                            .label("File")
                            .item(sapid::menu_item("new").label("New file"))
                            .item(sapid::menu_item("save").label("Save")),
                        sapid::menubar_menu("edit")
                            .label("Edit")
                            .item(sapid::menu_item("undo").label("Undo"))
                            .item(sapid::menu_item("redo").label("Redo")),
                    ]),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        format!("Opened menu: {}", open.get().as_deref().unwrap_or("none"))
                    })),
            ))
            .show_in(scope)
    })
}
