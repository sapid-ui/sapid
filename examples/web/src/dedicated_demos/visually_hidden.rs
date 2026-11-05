use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shown = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .fill((
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Info.drawing(theme.on_accent))
                            .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.))),
                        sapid::visually_hidden().content("Open project information"),
                    ))
                    .on_press(move || shown.revise(|v| *v = !*v)),
                sapid::text().theme(theme.clone()).content(shown.map(|v| {
                    if *v {
                        "The project is shared with your design team.".to_owned()
                    } else {
                        "Use the information action to read project details.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
