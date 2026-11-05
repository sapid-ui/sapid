use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let angle = scope.cage(90.);
        let accept = scope.cage(true);
        let coarse = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Accept angle changes")
                        .checked(accept)
                        .on_checked_change(move |v| accept.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Use 15 degree steps")
                        .checked(coarse)
                        .on_checked_change(move |v| coarse.revise(|old| *old = v)),
                )),
                sapid::angle_slider("controlled-angle")
                    .theme(theme.clone())
                    .name("Controlled rotation")
                    .value(angle)
                    .step(coarse.map(|v| if *v { 15. } else { 1. }))
                    .page_step(45.)
                    .on_value_change(move |v| {
                        if accept.get() {
                            angle.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(angle.map(|v| format!("Accepted rotation: {v:.0}°"))),
            ))
            .show_in(scope)
    })
}
