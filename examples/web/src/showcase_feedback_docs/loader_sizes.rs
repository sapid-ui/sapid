use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(20.).fill((
        sapid::group().theme(theme.clone()).gap(16.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .map(|size| sapid::loader().theme(theme.clone()).size(size).decorative())
            .collect::<Vec<_>>(),
        ),
        sapid::loader()
            .theme(theme.clone())
            .diameter(56.)
            .thickness(5.)
            .color(theme.warning)
            .name("Loading a large report"),
    ))
}
