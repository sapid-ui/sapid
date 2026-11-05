use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(28.).fill(
        [
            sapid::LoaderType::Oval,
            sapid::LoaderType::Bars,
            sapid::LoaderType::Dots,
        ]
        .into_iter()
        .map(|kind| {
            sapid::group().theme(theme.clone()).gap(8.).fill((
                sapid::loader().theme(theme.clone()).kind(kind).decorative(),
                sapid::text()
                    .theme(theme.clone())
                    .content(format!("{kind:?}")),
            ))
        })
        .collect::<Vec<_>>(),
    )
}
