use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroller("destinations")
        .area(|area| {
            area.theme(theme.clone())
                .name("Browse destinations")
                .style(Style::new().height(Length::Px(132.)))
        })
        .content(move |_| {
            sapid::group()
                .theme(theme.clone())
                .wrap(glory::style::FlexWrap::NoWrap)
                .gap(12.)
                .fill(
                    [
                        "Kyoto",
                        "Lisbon",
                        "Reykjavik",
                        "Copenhagen",
                        "Florence",
                        "Vancouver",
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(index, city)| {
                        sapid::paper()
                            .theme(theme.clone())
                            .style(
                                Style::new()
                                    .width(Length::Px(140.))
                                    .min_width(Length::Px(140.))
                                    .height(Length::Px(116.))
                                    .padding(Edges::all(16.)),
                            )
                            .fill(
                                sapid::stack().theme(theme.clone()).gap(8.).fill((
                                    sapid::text()
                                        .theme(theme.clone())
                                        .content(format!("0{}", index + 1)),
                                    sapid::text().theme(theme.clone()).content(city),
                                )),
                            )
                    })
                    .collect::<Vec<_>>(),
                )
        })
}
