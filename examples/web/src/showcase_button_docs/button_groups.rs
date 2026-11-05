use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::button_group()
            .theme(theme.clone())
            .name("Document actions")
            .button(sapid::button().label("Preview"))
            .button(sapid::button().label("Publish")),
        sapid::button_group()
            .theme(theme.clone())
            .name("Editor toolbar")
            .toolbar(true)
            .button(sapid::button().label("Undo"))
            .button(sapid::button().label("Redo").enabled(false))
            .button(sapid::button().label("History")),
        sapid::button_group()
            .theme(theme)
            .name("Export formats")
            .orientation(glory::render::Orientation::Vertical)
            .button(sapid::button().label("PDF"))
            .button(sapid::button().label("Text")),
    ))
}
