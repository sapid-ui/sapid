use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(24.).fill(
        [
            sapid::SeparatorVariant::Solid,
            sapid::SeparatorVariant::Dashed,
            sapid::SeparatorVariant::Dotted,
        ]
        .into_iter()
        .map(|variant| {
            sapid::separator()
                .theme(theme.clone())
                .variant(variant)
                .thickness(2.)
                .name(format!("{variant:?} boundary"))
        })
        .collect::<Vec<_>>(),
    )
}
