use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let collapsed = scope.cage(true);
        let accepted = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept navigation changes")
                    .checked(accepted)
                    .on_checked_change(move |v| accepted.revise(|old| *old = v)),
                sapid::app_shell("docs-shell-collapsed")
                    .theme(theme.clone())
                    .breakpoint(560.)
                    .style(Style::new().min_height(Length::Px(240.)))
                    .header(|context| {
                        sapid::text()
                            .theme(context.theme)
                            .content("Compact navigation")
                    })
                    .sidebar(
                        sapid::sidebar("docs-compact-nav")
                            .width(180.)
                            .rail_width(64.)
                            .collapsed(collapsed)
                            .on_collapsed_change(move |v| {
                                if accepted.get() {
                                    collapsed.revise(|old| *old = v);
                                }
                            })
                            .content(|context| {
                                widget(move |scope| {
                                    let collapsed = context.sidebar.collapsed();
                                    sapid::stack()
                                        .theme(context.theme.clone())
                                        .gap(14.)
                                        .style(Style::new().padding(Edges::all(12.)))
                                        .fill((
                                            sapid::text().theme(context.theme.clone()).content(
                                                scope.bond(move || {
                                                    if collapsed.get() { "O" } else { "Overview" }
                                                        .to_owned()
                                                }),
                                            ),
                                            sapid::text().theme(context.theme.clone()).content(
                                                scope.bond(move || {
                                                    if collapsed.get() { "P" } else { "Projects" }
                                                        .to_owned()
                                                }),
                                            ),
                                        ))
                                        .show_in(scope)
                                })
                            }),
                    )
                    .main(|context| {
                        sapid::text()
                            .theme(context.theme)
                            .content("Expand the rail with the menu icon.")
                    }),
            ))
            .show_in(scope)
    })
}
