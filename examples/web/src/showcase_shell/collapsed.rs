use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("docs-shell-collapsed")
        .theme(theme)
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
                .default_collapsed(true)
                .content(|context| {
                    widget(move |scope| {
                        let collapsed = context.sidebar.collapsed();
                        sapid::stack()
                            .theme(context.theme.clone())
                            .gap(14.)
                            .style(Style::new().padding(Edges::all(12.)))
                            .fill((
                                sapid::text()
                                    .theme(context.theme.clone())
                                    .content(scope.bond(move || {
                                        if collapsed.get() { "O" } else { "Overview" }.to_owned()
                                    })),
                                sapid::text()
                                    .theme(context.theme.clone())
                                    .content(scope.bond(move || {
                                        if collapsed.get() { "P" } else { "Projects" }.to_owned()
                                    })),
                            ))
                            .show_in(scope)
                    })
                }),
        )
        .main(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Expand the rail with the menu icon.")
        })
}
