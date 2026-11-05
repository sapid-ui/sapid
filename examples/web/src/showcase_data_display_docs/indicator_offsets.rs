use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(32.).fill(
        [
            (-6., 12., None),
            (0., 22., Some("8")),
            (10., 30., Some("99")),
        ]
        .into_iter()
        .map(|(offset, diameter, label)| {
            let marker = sapid::indicator()
                .theme(theme.clone())
                .offset(offset)
                .diameter(diameter)
                .color(Color(202, 65, 90, 255))
                .name(format!("Status offset {offset}"));
            let marker = if let Some(label) = label {
                marker.label(label)
            } else {
                marker
            };
            marker.fill(
                sapid::avatar("Jamie Chen")
                    .theme(theme.clone())
                    .size(sapid::Size::ExtraLarge)
                    .fallback("JC"),
            )
        })
        .collect::<Vec<_>>(),
    )
}
