use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill(
        [sapid::AvatarShape::Circle, sapid::AvatarShape::Square]
            .into_iter()
            .map(|shape| {
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
                        sapid::avatar("Jamie Chen")
                            .theme(theme.clone())
                            .size(size)
                            .shape(shape)
                            .fallback("JC")
                    })
                    .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
    )
}
