use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::menubar("docs-composition")
        .theme(theme)
        .name("Composed menu")
        .menus([
            sapid::menubar_menu("file")
                .label("File")
                .width(220.)
                .item(sapid::menu_item("new").label("New file"))
                .item(sapid::menu_item("open").label("Open…"))
                .separator("save-actions")
                .item(sapid::menu_item("save").label("Save")),
            sapid::menubar_menu("edit")
                .label("Edit")
                .item(sapid::menu_item("undo").label("Undo"))
                .item(sapid::menu_item("redo").label("Redo")),
        ])
}
