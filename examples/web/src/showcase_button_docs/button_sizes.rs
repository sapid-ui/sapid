use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            sapid::Size::ExtraSmall,
            sapid::Size::Small,
            sapid::Size::Medium,
            sapid::Size::Large,
            sapid::Size::ExtraLarge,
        ]
        .into_iter()
        .enumerate()
        .map(|(i, size)| {
            sapid::group().theme(theme.clone()).gap(12.).fill((
                sapid::button()
                    .theme(theme.clone())
                    .size(size)
                    .label(format!("Size {}", i + 1)),
                sapid::button()
                    .theme(theme.clone())
                    .size(size)
                    .compact(true)
                    .label(format!("Compact {}", i + 1)),
            ))
        })
        .collect::<Vec<_>>(),
    )
}
