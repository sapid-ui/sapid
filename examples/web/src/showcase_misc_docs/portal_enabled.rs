use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::portal_context(scope, "enabled-project-summary")?;
        let enabled = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Attach the project summary")
                    .checked(enabled)
                    .on_checked_change(move |v| enabled.revise(|s| *s = v)),
                sapid::paper().theme(theme.clone()).bordered(true).fill(
                    sapid::portal_target(&context, "summary").fill(
                        sapid::title(4)
                            .theme(theme.clone())
                            .weight(400.)
                            .content("Project summary"),
                    ),
                ),
                sapid::portal(&context, "summary")
                    .enabled(enabled)
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::text()
                                .theme(theme.clone())
                                .content("The design review is ready for your team.")
                        }
                    }),
            ))
            .show_in(scope)
    })
}
