use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("project-grid")
        .theme(theme.clone())
        .columns(3)
        .gap(12.)
        .item(
            sapid::grid_item("overview").span(2).fill(
                sapid::card().theme(theme.clone()).fill(
                    sapid::text()
                        .theme(theme.clone())
                        .content("Project overview occupies two columns."),
                ),
            ),
        )
        .item(
            sapid::grid_item("status").fill(
                sapid::card()
                    .theme(theme.clone())
                    .fill(sapid::text().theme(theme.clone()).content("Status")),
            ),
        )
        .item(
            sapid::grid_item("activity").span(3).fill(
                sapid::card().theme(theme.clone()).fill(
                    sapid::text()
                        .theme(theme)
                        .content("Recent activity uses the full row."),
                ),
            ),
        )
}
