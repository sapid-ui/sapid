use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            sapid::MarqueeDirection::Left,
            sapid::MarqueeDirection::Right,
        ]
        .into_iter()
        .map(|direction| {
            sapid::marquee()
                .theme(theme.clone())
                .direction(direction)
                .period_millis(14000u32)
                .text(TEXT_1)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(glory::style::Limit::Px(360.)),
                )
        })
        .collect::<Vec<_>>(),
    )
}

const TEXT_1: &str =
    "A calmer workspace · Thoughtful defaults · Clear next steps · Shared project notes";
