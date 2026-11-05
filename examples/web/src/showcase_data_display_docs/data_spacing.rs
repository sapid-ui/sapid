use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(24.).fill(
        [(72., 4., "Compact facts"), (110., 16., "Spacious facts")]
            .into_iter()
            .map(|(width, gap, name)| {
                sapid::data_list()
                    .theme(theme.clone())
                    .label_width(width)
                    .gap(gap)
                    .name(name)
                    .item(
                        sapid::data_list_item()
                            .term("Owner")
                            .fill(sapid::text().theme(theme.clone()).content("Ava Patel")),
                    )
                    .item(
                        sapid::data_list_item().term("Region").fill(
                            sapid::text()
                                .theme(theme.clone())
                                .content("Europe / Central"),
                        ),
                    )
            })
            .collect::<Vec<_>>(),
    )
}
