use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::toolbar("vertical-tools")
            .theme(theme.clone())
            .name("Vertical project actions")
            .orientation(glory::render::Orientation::Vertical)
            .group(sapid::toolbar_group("actions").buttons([
                sapid::button().label("Review"),
                sapid::button().label("Preview"),
                sapid::button().label("Publish"),
            ])),
        sapid::toolbar("rtl-tools")
            .theme(theme.clone())
            .name("RTL project actions")
            .direction(glory::render::Direction::Rtl)
            .group(sapid::toolbar_group("actions").buttons([
                sapid::button().label("First action"),
                sapid::button().label("Second action"),
                sapid::button().label("Third action"),
            ])),
    ))
}
