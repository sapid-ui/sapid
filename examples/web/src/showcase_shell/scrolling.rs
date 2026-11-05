use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("docs-shell-scrolling")
        .theme(theme)
        .breakpoint(260.)
        .main_scrollable(true)
        .style(Style::new().height(Length::Px(320.)))
        .header(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Project library")
        })
        .sidebar(
            sapid::sidebar("docs-section-nav")
                .width(132.)
                .rail_width(48.)
                .content(|context| {
                    let theme = context.theme;
                    glory::ui::group().fill((
                        sapid::shell_section()
                            .theme(theme.clone())
                            .style(Style::new().padding(Edges::all(10.)))
                            .fill(sapid::text().theme(theme.clone()).content("Collections")),
                        sapid::shell_section()
                            .theme(theme.clone())
                            .name("Project collections")
                            .scrollable(true)
                            .style(Style::new().padding(Edges::all(10.)))
                            .fill(
                                (1..=24)
                                    .map(|n| {
                                        sapid::text()
                                            .theme(theme.clone())
                                            .content(format!("Collection {n}"))
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                        sapid::shell_section()
                            .theme(theme.clone())
                            .class("docs-nav-footer")
                            .style(Style::new().padding(Edges::all(10.)))
                            .fill(sapid::button().theme(theme).label("New folder")),
                    ))
                }),
        )
        .main(|context| {
            sapid::stack().theme(context.theme.clone()).gap(16.).fill(
                (1..=16)
                    .map(|n| {
                        sapid::card().theme(context.theme.clone()).fill(
                            sapid::text()
                                .theme(context.theme.clone())
                                .content(format!("Project {n}")),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .footer(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Your library is up to date")
        })
}
