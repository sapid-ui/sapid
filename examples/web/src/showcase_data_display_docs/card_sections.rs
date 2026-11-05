use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::card()
        .theme(theme.clone())
        .title("Project overview")
        .description("Design system rollout")
        .header(sapid::badge().theme(theme.clone()).content("In progress"))
        .fill(
            sapid::data_list()
                .theme(theme.clone())
                .label_width(90.)
                .item(
                    sapid::data_list_item()
                        .term("Owner")
                        .fill(sapid::text().theme(theme.clone()).content("Ava Patel")),
                )
                .item(
                    sapid::data_list_item()
                        .term("Due date")
                        .fill(sapid::text().theme(theme.clone()).content("November 30")),
                ),
        )
        .footer(
            sapid::text()
                .theme(theme.clone())
                .content("Updated a few minutes ago"),
        )
}
