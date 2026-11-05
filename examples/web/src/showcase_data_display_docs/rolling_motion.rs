use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(75.);
        let paused = scope.cage(false);
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(value)
                    .paused(paused)
                    .animated(animated),
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(value)
                    .duration_millis(0u16),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate comparison")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|s| *s = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Pause comparison")
                    .checked(paused)
                    .on_checked_change(move |v| paused.revise(|s| *s = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Advance comparison")
                    .on_press(move || value.revise(|v| *v += 25.)),
            ))
            .show_in(scope)
    })
}
