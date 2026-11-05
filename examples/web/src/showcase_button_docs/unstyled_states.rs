use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::button()
            .theme(theme.clone())
            .unstyled()
            .label("Unstyled native action"),
        sapid::button()
            .theme(theme.clone())
            .unstyled()
            .label("Unavailable unstyled action")
            .enabled(false),
    ))
}
