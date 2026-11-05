use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let present = scope.cage(true);
        let animated = scope.cage(true);
        let active = scope.cage(Some("project".to_owned()));
        let body = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Keep active indicator target mounted")
                    .checked(present)
                    .on_checked_change(move |v| present.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate workspace indicator")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|old| *old = v)),
                sapid::floating_indicator("lifetime-indicator")
                    .theme(theme.clone())
                    .active(active)
                    .animated(animated)
                    .duration(320u16)
                    .color(theme.warning)
                    .content(move |context| {
                        let body = body.clone();
                        view().fill(when(present, move || {
                            sapid::floating_indicator_target(context.clone(), "project")
                                .fill(sapid::button().theme(body.clone()).label("Current project"))
                        }))
                    }),
            ))
            .show_in(scope)
    })
}
