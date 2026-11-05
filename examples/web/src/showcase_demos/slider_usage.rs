use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(40.);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Volume: {v:.0}%"))),
                sapid::slider("volume")
                    .theme(theme.clone())
                    .name("Volume")
                    .value(value)
                    .value_text(value.map(|v| Some(format!("{v:.0} percent"))))
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| value.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
