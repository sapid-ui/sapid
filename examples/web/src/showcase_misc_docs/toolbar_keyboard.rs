use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::toolbar("keyboard-tools")
        .theme(theme.clone())
        .name("Review actions")
        .group(sapid::toolbar_group("review").buttons([
            sapid::button().label("Previous comment"),
            sapid::button().label("Unavailable action").enabled(false),
            sapid::button().label("Next comment"),
            sapid::button().label("Resolve comment"),
        ]))
        .group(sapid::toolbar_group("document").button(sapid::button().label("Preview document")))
}
