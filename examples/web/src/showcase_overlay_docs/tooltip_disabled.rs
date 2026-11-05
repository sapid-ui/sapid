use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let enabled = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::tooltip("disabled-publish")
                    .theme(theme.clone())
                    .label("Publish project")
                    .text("Publish the prepared project.")
                    .enabled(enabled),
                sapid::text()
                    .theme(theme.clone())
                    .content("Complete the required project details before publishing."),
                sapid::tooltip("publish-explanation")
                    .theme(theme.clone())
                    .label("Why is publishing unavailable?")
                    .text("Complete the required project details before publishing."),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Project details are complete")
                    .checked(enabled)
                    .on_checked_change(move |v| enabled.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
