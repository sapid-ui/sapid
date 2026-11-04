use glory::prelude::*;
use glory::render::{AnchorAlign, AnchorPlacement, AnchorSide};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::menubar("docs-position")
        .theme(theme)
        .name("Position example")
        .position(AnchorPlacement::new(AnchorSide::Bottom, AnchorAlign::End))
        .menus([
            sapid::menubar_menu("file")
                .label("Bottom end")
                .width(260.)
                .item(sapid::menu_item("new").label("New file")),
            sapid::menubar_menu("edit")
                .label("Top start")
                .width(180.)
                .position(AnchorPlacement::new(AnchorSide::Top, AnchorAlign::Start))
                .item(sapid::menu_item("undo").label("Undo")),
        ])
}
