use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(24.).fill(
        [
            sapid::DataListOrientation::Horizontal,
            sapid::DataListOrientation::Vertical,
        ]
        .into_iter()
        .map(|orientation| {
            sapid::data_list()
                .theme(theme.clone())
                .orientation(orientation)
                .label_width(90.)
                .name(format!("{orientation:?} project facts"))
                .item(
                    sapid::data_list_item().term("Name").fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("Mountain studio"),
                    ),
                )
                .item(
                    sapid::data_list_item().term("Purpose").fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("A calm workspace for team projects and shared research."),
                    ),
                )
        })
        .collect::<Vec<_>>(),
    )
}
