use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::pagination("numbers-only")
            .theme(theme.clone())
            .name("Numbers only")
            .total(Some(6))
            .with_controls(false)
            .with_edges(false),
        sapid::pagination("previous-next")
            .theme(theme.clone())
            .name("Previous and next")
            .total(Some(6))
            .default_page(3)
            .with_edges(false),
        sapid::pagination("disabled-pages")
            .theme(theme)
            .name("Disabled pages")
            .total(Some(6))
            .default_page(3)
            .enabled(false),
    ))
}
