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
        .map(|size| {
            sapid::pill()
                .theme(theme.clone())
                .size(size)
                .content("Rust")
                .removable(format!("Remove Rust {size:?}"))
        })
        .collect::<Vec<_>>(),
    )
}
