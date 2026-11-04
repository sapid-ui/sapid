use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("docs-shell-responsive")
        .theme(theme)
        .breakpoint(560.)
        .style(Style::new().min_height(Length::Px(280.)))
        .header(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Responsive workspace")
        })
        .sidebar(
            sapid::sidebar("docs-responsive-nav")
                .width(180.)
                .name("Responsive navigation")
                .content(|context| {
                    sapid::stack()
                        .theme(context.theme.clone())
                        .gap(14.)
                        .style(Style::new().padding(Edges::all(12.)))
                        .fill((
                            sapid::text()
                                .theme(context.theme.clone())
                                .content("Navigation"),
                            sapid::text_field()
                                .theme(context.theme)
                                .name("Navigation draft")
                                .placeholder("Keep a note…"),
                        ))
                }),
        )
        .main(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Resize the page, then open navigation with the menu icon.")
        })
}
