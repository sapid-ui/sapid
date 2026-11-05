use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(10.).fill((
        sapid::stack().theme(theme.clone()).gap(6.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .map(|size| {
                sapid::text()
                    .theme(theme.clone())
                    .size(size)
                    .content(format!("{size:?} text"))
            })
            .collect::<Vec<_>>(),
        ),
        sapid::text()
            .theme(theme.clone())
            .font_size(20.)
            .line_height(30.)
            .weight(500.)
            .color(theme.accent_text)
            .content("A deliberate emphasis"),
        sapid::text()
            .theme(theme.clone())
            .variant(sapid::TextVariant::Muted)
            .content("A quiet supporting detail"),
    ))
}
