use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::group().theme(theme.clone()).gap(10.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .map(|size| {
                sapid::badge()
                    .theme(theme.clone())
                    .size(size)
                    .content(format!("{size:?}"))
            })
            .collect::<Vec<_>>(),
        ),
        sapid::badge()
            .theme(theme.clone())
            .left_section(sapid::Icon::Info)
            .style(Style::new().max_width(glory::style::Limit::Px(210.)))
            .content("A long descriptive status for a shared workspace"),
    ))
}
