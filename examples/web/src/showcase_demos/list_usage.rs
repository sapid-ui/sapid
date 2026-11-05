use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::text_list()
        .theme(theme.clone())
        .name("Project launch checklist")
        .items(
            [
                "Review the project brief",
                "Invite the design team",
                "Share the first prototype",
            ]
            .into_iter()
            .map(|label| {
                sapid::text_list_item().fill(sapid::text().theme(theme.clone()).content(label))
            }),
        )
}
