use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::button()
            .theme(theme.clone())
            .unstyled()
            .label("Perform an application action"),
        sapid::anchor()
            .theme(theme)
            .href("https://mantine.dev/core/unstyled-button/")
            .label("Read the reference documentation"),
    ))
}
