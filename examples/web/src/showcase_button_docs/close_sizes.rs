use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
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
            sapid::close_button()
                .theme(theme.clone())
                .size(size)
                .name(format!("Close size {}", i + 1))
        })
        .collect::<Vec<_>>(),
    )
}
