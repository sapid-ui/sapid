use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(26.).fill(
        [
            sapid::IndicatorPosition::TopLeft,
            sapid::IndicatorPosition::TopCenter,
            sapid::IndicatorPosition::TopRight,
            sapid::IndicatorPosition::MiddleLeft,
            sapid::IndicatorPosition::MiddleCenter,
            sapid::IndicatorPosition::MiddleRight,
            sapid::IndicatorPosition::BottomLeft,
            sapid::IndicatorPosition::BottomCenter,
            sapid::IndicatorPosition::BottomRight,
        ]
        .into_iter()
        .map(|position| {
            sapid::indicator()
                .theme(theme.clone())
                .position(position)
                .name(format!("{position:?} marker"))
                .fill(
                    sapid::paper()
                        .theme(theme.clone())
                        .style(Style::new().width(Length::Px(62.)).height(Length::Px(52.)))
                        .fill(sapid::text().theme(theme.clone()).content(match position {
                            sapid::IndicatorPosition::TopLeft => "TL",
                            sapid::IndicatorPosition::TopCenter => "TC",
                            sapid::IndicatorPosition::TopRight => "TR",
                            sapid::IndicatorPosition::MiddleLeft => "ML",
                            sapid::IndicatorPosition::MiddleCenter => "MC",
                            sapid::IndicatorPosition::MiddleRight => "MR",
                            sapid::IndicatorPosition::BottomLeft => "BL",
                            sapid::IndicatorPosition::BottomCenter => "BC",
                            sapid::IndicatorPosition::BottomRight => "BR",
                        })),
                )
        })
        .collect::<Vec<_>>(),
    )
}
