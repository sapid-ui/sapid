use glory::prelude::*;
use glory::render::Direction;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::text().theme(theme.clone()).content("Use Up/Down, Home/End or type an item's initial letters. In RTL, Left enters a submenu and Right returns."),
        sapid::group().theme(theme.clone()).fill((
            sapid::dropdown_menu("rtl-actions").theme(theme.clone()).label("RTL actions")
                .direction(Direction::Rtl).loop_focus(false).width(220.)
                .item(sapid::menu_item("archive").label("Archive"))
                .item(sapid::menu_item("disabled").label("Unavailable action").enabled(false))
                .submenu(sapid::menu_submenu("export").label("Export")
                    .item(sapid::menu_item("pdf").label("PDF"))),
            sapid::dropdown_menu("disabled-menu").theme(theme.clone()).label("Disabled menu").enabled(false)
                .item(sapid::menu_item("unavailable").label("Unavailable")),
            sapid::dropdown_menu("unstyled-menu").theme(theme).label("Unstyled menu").unstyled().width(220.)
                .item(sapid::menu_label("application", "Application commands"))
                .item(sapid::menu_item("settings").label("Settings"))
                .item(sapid::menu_item("about").label("About")),
        )),
    ))
}
