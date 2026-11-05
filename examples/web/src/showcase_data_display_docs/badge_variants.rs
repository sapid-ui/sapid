use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            sapid::BadgeVariant::Soft,
            sapid::BadgeVariant::Solid,
            sapid::BadgeVariant::Outline,
            sapid::BadgeVariant::Dot,
        ]
        .into_iter()
        .map(|variant| {
            sapid::badge()
                .theme(theme.clone())
                .variant(variant)
                .color(Color(238, 173, 35, 255))
                .content(format!("{variant:?}"))
        })
        .collect::<Vec<_>>(),
    )
}
