use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [false, true]
            .into_iter()
            .map(|ordered| {
                sapid::text_list()
                    .theme(theme.clone())
                    .ordered(ordered)
                    .name(if ordered {
                        "Ordered launch steps"
                    } else {
                        "Unordered project notes"
                    })
                    .items(
                        ["Read the brief", "Prepare the draft", "Share with the team"]
                            .into_iter()
                            .map(|label| {
                                sapid::text_list_item()
                                    .fill(sapid::text().theme(theme.clone()).content(label))
                            }),
                    )
            })
            .collect::<Vec<_>>(),
    )
}
