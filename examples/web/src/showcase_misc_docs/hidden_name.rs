use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let done = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .fill((
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Check.drawing(theme.on_accent))
                            .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.))),
                        sapid::visually_hidden().content("Approve project review"),
                    ))
                    .on_press(move || done.revise(|v| *v = true)),
                sapid::text().theme(theme.clone()).content(done.map(|v| {
                    if *v {
                        "Project review approved.".to_owned()
                    } else {
                        "The review is waiting for approval.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
