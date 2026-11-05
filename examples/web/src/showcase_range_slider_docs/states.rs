use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage([20., 80.]);
        let coarse = scope.cage(true);
        let readonly = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use five-unit range steps")
                    .checked(coarse)
                    .on_checked_change(move |v| coarse.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Lock range")
                    .checked(readonly)
                    .on_checked_change(move |v| readonly.revise(|old| *old = v)),
                sapid::range_slider("live-range")
                    .theme(theme.clone())
                    .value(value)
                    .minimum_gap(10.)
                    .step(coarse.map(|v| if *v { 5. } else { 1. }))
                    .page_step(20.)
                    .readonly(readonly)
                    .labels(["Live lower".into(), "Live upper".into()])
                    .style(Style::new().width(Length::Percent(100.)))
                    .show_value_labels(false)
                    .on_value_change(move |v| value.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Range: {:.0} – {:.0}", v[0], v[1]))),
                sapid::range_slider("disabled-range")
                    .theme(theme.clone())
                    .default_value([20., 80.])
                    .labels(["Unavailable lower".into(), "Unavailable upper".into()])
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
