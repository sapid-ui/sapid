use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(40.);
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
                    .label("Use five-unit steps")
                    .checked(coarse)
                    .on_checked_change(move |v| coarse.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Lock volume")
                    .checked(readonly)
                    .on_checked_change(move |v| readonly.revise(|old| *old = v)),
                sapid::slider("live-volume")
                    .theme(theme.clone())
                    .name("Configurable volume")
                    .value(value)
                    .step(coarse.map(|v| if *v { 5. } else { 1. }))
                    .page_step(20.)
                    .readonly(readonly)
                    .style(Style::new().width(Length::Percent(100.)))
                    .value_text(value.map(|v| Some(format!("{v:.0} percent"))))
                    .on_value_change(move |v| value.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Volume: {v:.0}%"))),
                sapid::slider("unavailable-volume")
                    .theme(theme.clone())
                    .name("Unavailable volume")
                    .default_value(40.)
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
