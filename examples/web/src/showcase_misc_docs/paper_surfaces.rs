use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill(
        [
            sapid::ShadowSize::None,
            sapid::ShadowSize::Small,
            sapid::ShadowSize::Medium,
            sapid::ShadowSize::Large,
        ]
        .into_iter()
        .map(|shadow| {
            sapid::paper()
                .theme(theme.clone())
                .bordered(true)
                .shadow(shadow)
                .style(Style::new().padding(Edges::all(16.)))
                .fill(
                    sapid::text()
                        .theme(theme.clone())
                        .content(format!("{shadow:?} shadow")),
                )
        })
        .collect::<Vec<_>>(),
    )
}
