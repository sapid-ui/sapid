use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let action = scope.cage("Choose a command".to_owned());
        let item = move |key: &str, label: &str| {
            let message = label.to_owned();
            sapid::menu_item(key)
                .label(label.to_owned())
                .on_activate(move || action.revise(|value| *value = message.clone()))
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::menubar("showcase-editor")
                    .theme(theme.clone())
                    .name("Editor menu")
                    .menus([
                        sapid::menubar_menu("file")
                            .label("File")
                            .item(item("new", "New file"))
                            .item(item("window", "New window"))
                            .submenu(
                                sapid::menu_submenu("recent")
                                    .label("Open recent")
                                    .item(item("alpha", "project-alpha"))
                                    .item(item("beta", "project-beta")),
                            )
                            .separator("file-actions")
                            .item(item("save", "Save"))
                            .item(item("save-as", "Save as…")),
                        sapid::menubar_menu("edit")
                            .label("Edit")
                            .item(item("undo", "Undo"))
                            .item(item("redo", "Redo"))
                            .separator("clipboard")
                            .item(item("cut", "Cut"))
                            .item(item("copy", "Copy"))
                            .item(sapid::menu_item("paste").label("Paste").enabled(false)),
                        sapid::menubar_menu("help")
                            .label("Help")
                            .item(item("docs", "Documentation"))
                            .item(item("shortcuts", "Keyboard shortcuts"))
                            .item(item("about", "About")),
                    ]),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || action.get()))
                    .class("site-menubar-action"),
            ))
            .show_in(scope)
    })
}
