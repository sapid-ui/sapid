use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill((
        sapid::toggle()
            .theme(theme.clone())
            .label("Pinned document")
            .default_pressed(true),
        sapid::toggle()
            .theme(theme.clone())
            .label("Unavailable pinned document")
            .default_pressed(true)
            .enabled(false),
        sapid::toggle()
            .theme(theme)
            .label("Unavailable unselected document")
            .enabled(false),
    ))
}
