use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::group().theme(theme.clone()).gap(12.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .map(|size| {
                sapid::theme_icon(sapid::Icon::Check)
                    .theme(theme.clone())
                    .size(size)
                    .name(format!("Check {size:?}"))
            })
            .collect::<Vec<_>>(),
        ),
        sapid::group().theme(theme.clone()).gap(12.).fill(
            [12., 20., 28.]
                .into_iter()
                .map(|extent| {
                    sapid::theme_icon(sapid::Icon::Info)
                        .theme(theme.clone())
                        .size(sapid::Size::ExtraLarge)
                        .icon_size(extent)
                        .name(format!("Information icon {extent}px"))
                })
                .collect::<Vec<_>>(),
        ),
    ))
}
