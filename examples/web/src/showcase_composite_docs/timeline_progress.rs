use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let progress = scope.cage(sapid::TimelineProgress::Active("review".to_owned()));
        sapid::stack()
            .theme(theme.clone())
            .gap(20.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Complete release")
                        .on_press(move || {
                            progress.revise(|old| *old = sapid::TimelineProgress::Complete)
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restart review")
                        .on_press(move || {
                            progress.revise(|old| {
                                *old = sapid::TimelineProgress::Active("review".into())
                            })
                        }),
                )),
                sapid::timeline("release-review")
                    .theme(theme.clone())
                    .progress(progress)
                    .item(
                        sapid::timeline_item("design")
                            .title("Design")
                            .description("Agree on the scope"),
                    )
                    .item(
                        sapid::timeline_item("review")
                            .title("Review")
                            .description("Validate the implementation"),
                    )
                    .item(
                        sapid::timeline_item("publish")
                            .title("Publish")
                            .description("Prepare the release"),
                    ),
            ))
            .show_in(scope)
    })
}
