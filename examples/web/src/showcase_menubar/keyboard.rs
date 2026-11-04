use glory::prelude::*;
use glory::render::Direction;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::text().theme(theme.clone()).content(
            "Focus a target and try arrows, Home, End and character keys. This bar uses RTL.",
        ),
        sapid::menubar("docs-keyboard")
            .theme(theme)
            .name("RTL keyboard example")
            .direction(Direction::Rtl)
            .menus([
                sapid::menubar_menu("file")
                    .label("File")
                    .item(sapid::menu_item("new").label("New file")),
                sapid::menubar_menu("disabled")
                    .label("Disabled")
                    .enabled(false)
                    .item(sapid::menu_item("unused").label("Unavailable")),
                sapid::menubar_menu("edit")
                    .label("Edit")
                    .item(sapid::menu_item("undo").label("Undo")),
                sapid::menubar_menu("view")
                    .label("View")
                    .item(sapid::menu_item("zoom").label("Zoom in")),
            ]),
    ))
}
