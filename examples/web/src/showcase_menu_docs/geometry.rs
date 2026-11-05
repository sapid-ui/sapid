use glory::prelude::*;
use glory::render::{AnchorAlign, AnchorPlacement, AnchorSide};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(16.).fill((
        sapid::dropdown_menu("top-menu")
            .theme(theme.clone())
            .label("Menu above the trigger")
            .width(260.)
            .position(AnchorPlacement::new(AnchorSide::Top, AnchorAlign::End).gap(8.))
            .item(sapid::menu_item("summary").label("Summary"))
            .submenu(
                sapid::menu_submenu("exports")
                    .label("Exports to the left")
                    .width(180.)
                    .position(AnchorPlacement::new(AnchorSide::Left, AnchorAlign::Start).gap(6.))
                    .item(sapid::menu_item("text").label("Plain text"))
                    .item(sapid::menu_item("markdown").label("Markdown")),
            ),
        sapid::dropdown_menu("right-menu")
            .theme(theme)
            .label("Menu beside the trigger")
            .width(220.)
            .position(AnchorPlacement::new(AnchorSide::Right, AnchorAlign::Start).gap(8.))
            .item(sapid::menu_item("details").label("Details"))
            .item(sapid::menu_item("history").label("History")),
    ))
}
