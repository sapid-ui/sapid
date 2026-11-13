use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(20.).fill((
        sapid::anchor()
            .theme(theme.clone())
            .label("Getting started")
            .href("#getting-started"),
        sapid::anchor()
            .theme(theme.clone())
            .label("Components")
            .href("?widget=button"),
        sapid::anchor()
            .theme(theme)
            .label("Source code")
            .href("https://github.com/taidge/sapid"),
    ))
}
