use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroller("fixed-distance")
        .scroll_step(180.)
        .control_size(28.)
        .previous_icon(text().content("←"))
        .next_icon(text().content("→"))
        .area(|area| {
            area.theme(theme.clone())
                .name("Fixed distance cards")
                .style(Style::new().height(Length::Px(90.)))
        })
        .content(move |_| {
            sapid::group()
                .theme(theme.clone())
                .wrap(glory::style::FlexWrap::NoWrap)
                .gap(10.)
                .fill(
                    (1..=12)
                        .map(|index| {
                            sapid::paper()
                                .theme(theme.clone())
                                .style(
                                    Style::new()
                                        .width(Length::Px(120.))
                                        .min_width(Length::Px(120.))
                                        .height(Length::Px(72.))
                                        .padding(Edges::all(16.)),
                                )
                                .fill(text().content(format!("Card {index}")))
                        })
                        .collect::<Vec<_>>(),
                )
        })
}
