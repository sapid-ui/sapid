use glory::prelude::*;
pub fn link(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme)
        .label("Open full-window workspace")
        .href("shell-window.html")
}
/// Mounted on a separate page so fixed regions never cover these documentation examples.
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("full-window-workspace")
        .theme(theme)
        .viewport()
        .breakpoint(600.)
        .header(|context| {
            sapid::group().theme(context.theme.clone()).fill((
                sapid::text()
                    .theme(context.theme.clone())
                    .content("Acme projects"),
                sapid::anchor()
                    .theme(context.theme)
                    .label("Back to AppShell documentation")
                    .href("showcase.html?widget=app-shell"),
            ))
        })
        .sidebar(
            sapid::sidebar("full-window-navigation")
                .width(180.)
                .scrollable(true)
                .content(|context| {
                    sapid::stack()
                        .theme(context.theme.clone())
                        .padding(Edges::all(12.))
                        .fill(
                            (1..=30)
                                .map(|n| {
                                    sapid::text()
                                        .theme(context.theme.clone())
                                        .content(format!("Workspace {n}"))
                                })
                                .collect::<Vec<_>>(),
                        )
                }),
        )
        .main(|context| {
            sapid::stack().theme(context.theme.clone()).gap(16.).fill((
                sapid::title(2)
                    .theme(context.theme.clone())
                    .content("Project overview"),
                sapid::text_field()
                    .theme(context.theme.clone())
                    .name("Project note")
                    .placeholder("Write a project note…"),
                sapid::stack().theme(context.theme.clone()).gap(16.).fill(
                    (1..=24)
                        .map(|n| {
                            sapid::card().theme(context.theme.clone()).fill(
                                sapid::text()
                                    .theme(context.theme.clone())
                                    .content(format!("Milestone {n}")),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            ))
        })
        .footer(|context| {
            sapid::text()
                .theme(context.theme)
                .content("All changes saved")
        })
}
