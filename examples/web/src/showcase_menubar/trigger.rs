use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::text()
            .theme(theme.clone())
            .content("Move over File, Edit or View. Arrow navigation stops at the ends."),
        sapid::menubar("docs-trigger")
            .theme(theme)
            .name("Hover menu")
            .open_on_hover(true)
            .loop_focus(false)
            .menus([
                sapid::menubar_menu("file")
                    .label("File")
                    .item(sapid::menu_item("new").label("New file")),
                sapid::menubar_menu("edit")
                    .label("Edit")
                    .item(sapid::menu_item("undo").label("Undo")),
                sapid::menubar_menu("view")
                    .label("View")
                    .item(sapid::menu_item("zoom").label("Zoom in")),
            ]),
    ))
}
