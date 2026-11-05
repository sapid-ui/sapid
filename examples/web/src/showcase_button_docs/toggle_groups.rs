use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::toggle_group("alignment")
            .theme(theme.clone())
            .name("Text alignment")
            .default_value(["left"])
            .item(sapid::toggle_item("left").label("Left"))
            .item(sapid::toggle_item("center").label("Center"))
            .item(sapid::toggle_item("right").label("Right")),
        sapid::toggle_group("formatting")
            .theme(theme.clone())
            .name("Text formatting")
            .mode(sapid::ToggleMode::Multiple)
            .default_value(["bold"])
            .item(sapid::toggle_item("bold").label("Bold"))
            .item(sapid::toggle_item("italic").label("Italic"))
            .item(
                sapid::toggle_item("underline")
                    .label("Underline")
                    .enabled(false),
            ),
    ))
}
