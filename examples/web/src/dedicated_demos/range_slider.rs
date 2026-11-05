use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage([20., 80.]);
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
                    .content(value.map(|v| format!("Price range: ${:.0} – ${:.0}", v[0], v[1]))),
                sapid::range_slider("price-range")
                    .theme(theme.clone())
                    .value(value)
                    .value_text(
                        value.map(|v| {
                            [Some(format!("${:.0}", v[0])), Some(format!("${:.0}", v[1]))]
                        }),
                    )
                    .show_value_labels(false)
                    .minimum_gap(10.)
                    .labels(["Minimum price".into(), "Maximum price".into()])
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| value.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
