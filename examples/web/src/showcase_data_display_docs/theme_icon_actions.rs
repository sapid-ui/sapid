use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let opened = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(12.).fill((
                    sapid::theme_icon(sapid::Icon::Info)
                        .theme(theme.clone())
                        .name("Project information"),
                    sapid::action_icon("Open project information")
                        .theme(theme.clone())
                        .icon(
                            glory::ui::graphic()
                                .drawing(sapid::Icon::Info.drawing(theme.accent_text))
                                .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.))),
                        )
                        .on_press(move || opened.revise(|v| *v = !*v)),
                )),
                sapid::text().theme(theme.clone()).content(opened.map(|v| {
                    if *v {
                        "Project information: this workspace is shared with your design team."
                            .to_owned()
                    } else {
                        "Use the information action to view details.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
