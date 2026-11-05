use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let hue = scope.cage(180.);
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept hue changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::hue_slider("controlled-hue")
                    .theme(theme.clone())
                    .name("Controlled hue")
                    .value(hue)
                    .step(15.)
                    .page_step(60.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(360.)),
                    )
                    .on_value_change(move |v| {
                        if accept.get() {
                            hue.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(hue.map(|v| format!("Accepted hue: {v:.0}°"))),
            ))
            .show_in(scope)
    })
}
