use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let help = scope.cage(false);
        let body_theme = theme.clone();
        sapid::empty_state()
            .theme(theme.clone())
            .title("No saved reports")
            .description("Save a report to reuse its filters and columns.")
            .fill(when(help, move || {
                sapid::text().theme(body_theme.clone()).content(
                    "Open a report, choose its filters, then save a copy from the report actions.",
                )
            }))
            .actions((
                sapid::button()
                    .theme(theme.clone())
                    .label("Show report help")
                    .on_press(move || help.revise(|v| *v = true)),
                sapid::anchor()
                    .theme(theme.clone())
                    .label("Read the empty state guide")
                    .href("https://mantine.dev/core/empty-state/"),
            ))
            .show_in(scope)
    })
}
