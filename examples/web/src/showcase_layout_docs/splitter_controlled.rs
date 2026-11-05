use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(45.);
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Accept resize proposals")
                        .checked(accept)
                        .on_checked_change(move |v| accept.revise(|old| *old = v)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reset panel sizes")
                        .on_press(move || value.revise(|old| *old = 45.)),
                )),
                sapid::splitter("docs-controlled-splitter")
                    .theme(theme.clone())
                    .name("Resize controlled panels")
                    .value(value)
                    .min(25.)
                    .max(75.)
                    .step(5.)
                    .style(Style::new().height(Length::Px(200.)))
                    .on_value_change(move |next| {
                        if accept.get() {
                            value.revise(|old| *old = next);
                        }
                    })
                    .first({
                        let theme = theme.clone();
                        move || {
                            sapid::box_()
                                .theme(theme.clone())
                                .padding(Edges::all(12.))
                                .fill(
                                    sapid::text_field()
                                        .theme(theme.clone())
                                        .name("Controlled panel note")
                                        .style(Style::new().width(Length::Percent(100.))),
                                )
                        }
                    })
                    .second({
                        let theme = theme.clone();
                        move || sapid::text().theme(theme.clone()).content("Preview")
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Editor width: {v:.0}%"))),
            ))
            .show_in(scope)
    })
}
