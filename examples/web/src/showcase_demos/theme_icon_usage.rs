use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(14.).fill((
        sapid::theme_icon(sapid::Icon::Check)
            .theme(theme.clone())
            .name("Project complete")
            .size(sapid::Size::Large)
            .variant(sapid::ThemeIconVariant::Solid),
        sapid::stack().theme(theme.clone()).gap(4.).fill((
            sapid::text()
                .theme(theme.clone())
                .content("Everything is ready"),
            sapid::text()
                .theme(theme.clone())
                .content("Your project passed its final review."),
        )),
    ))
}
