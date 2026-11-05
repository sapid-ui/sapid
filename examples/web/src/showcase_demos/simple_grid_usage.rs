use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::simple_grid("project-cards")
        .theme(theme.clone())
        .columns(3)
        .gap(12.)
        .items(
            [
                "Planning",
                "Design review with the project team",
                "Delivery",
            ]
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                sapid::grid_item(i.to_string()).fill(
                    sapid::card()
                        .theme(theme.clone())
                        .fill(sapid::text().theme(theme.clone()).content(label)),
                )
            }),
        )
}
