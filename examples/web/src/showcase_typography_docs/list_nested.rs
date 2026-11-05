use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::text_list()
        .theme(theme.clone())
        .ordered(true)
        .name("Release plan")
        .item(
            sapid::text_list_item().fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Prepare the release"),
            ),
        )
        .item(
            sapid::text_list_item().fill((
                sapid::text()
                    .theme(theme.clone())
                    .content("Review the details"),
                sapid::text_list().theme(theme.clone()).items(
                    ["Accessibility", "Documentation", "Browser behavior"]
                        .into_iter()
                        .map(|label| {
                            sapid::text_list_item()
                                .fill(sapid::text().theme(theme.clone()).content(label))
                        }),
                ),
            )),
        )
        .item(
            sapid::text_list_item().fill(
                sapid::text()
                    .theme(theme.clone())
                    .content("Publish the update"),
            ),
        )
}
