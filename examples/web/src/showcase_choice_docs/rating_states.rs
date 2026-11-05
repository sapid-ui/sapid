use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::rating("average-score")
            .theme(theme.clone())
            .name("Average score")
            .precision(sapid::RatingPrecision::Half)
            .default_value(3.5)
            .display_only(),
        sapid::rating("readonly-score")
            .theme(theme.clone())
            .name("Read-only score")
            .default_value(3.)
            .readonly(true),
        sapid::rating("disabled-score")
            .theme(theme.clone())
            .name("Disabled score")
            .default_value(2.)
            .enabled(false),
        sapid::stack().theme(theme.clone()).gap(12.).fill(Vec::from(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .map(|size| {
                sapid::rating(format!("size-{size:?}").to_lowercase().replace('_', "-"))
                    .theme(theme.clone())
                    .size(size)
                    .name(format!("Rating size {size:?}"))
                    .default_value(3.)
                    .display_only()
            }),
        )),
    ))
}
