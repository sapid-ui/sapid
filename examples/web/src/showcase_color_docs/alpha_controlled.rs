use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let alpha = scope.cage(0.5);
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept opacity changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::alpha_slider("controlled-opacity")
                    .theme(theme.clone())
                    .name("Controlled opacity")
                    .value(alpha)
                    .step(0.05)
                    .page_step(0.2)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(360.)),
                    )
                    .on_value_change(move |v| {
                        if accept.get() {
                            alpha.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(alpha.map(|v| format!("Accepted opacity: {:.0}%", v * 100.))),
            ))
            .show_in(scope)
    })
}
