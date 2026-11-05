use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::data_list()
        .theme(theme.clone())
        .label_width(96.)
        .name("Project facts")
        .item(
            sapid::data_list_item().term("Project").fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Mountain studio"),
            ),
        )
        .item(
            sapid::data_list_item().term("Status").fill(
                sapid::badge()
                    .theme(theme.clone())
                    .variant(sapid::BadgeVariant::Dot)
                    .content("In progress"),
            ),
        )
        .item(
            sapid::data_list_item()
                .term("Owner")
                .fill(sapid::text().theme(theme.clone()).content("Ava Patel")),
        )
        .item(sapid::data_list_item().term("Description").fill(
            sapid::text().theme(theme.clone()).content(
                "A shared workspace for design reviews, research notes and upcoming releases.",
            ),
        ))
}
