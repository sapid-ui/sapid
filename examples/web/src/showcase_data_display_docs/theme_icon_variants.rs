use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(14.).fill((
        sapid::theme_icon(sapid::Icon::Info)
            .theme(theme.clone())
            .variant(sapid::ThemeIconVariant::Soft)
            .name("Soft information"),
        sapid::theme_icon(sapid::Icon::Check)
            .theme(theme.clone())
            .variant(sapid::ThemeIconVariant::Solid)
            .name("Solid check"),
        sapid::theme_icon(sapid::Icon::Calendar)
            .theme(theme.clone())
            .variant(sapid::ThemeIconVariant::Outline)
            .name("Outlined calendar"),
        sapid::theme_icon(sapid::Icon::ArrowRight)
            .theme(theme.clone())
            .color(Color(80, 35, 120, 255))
            .style(Style {
                background: Some(Color(237, 215, 250, 255)),
                radius: Some(18.),
                ..Style::new()
            })
            .name("Purple forward icon"),
    ))
}
