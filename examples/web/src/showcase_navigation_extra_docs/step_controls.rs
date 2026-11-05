use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(0_usize);
        let accept = scope.cage(false);
        let skip = scope.cage(false);
        let enabled = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::stepper("controlled-workflow")
                    .theme(theme.clone())
                    .name("Controlled workflow")
                    .active(active)
                    .selectable(true)
                    .allow_skip(skip)
                    .enabled(enabled)
                    .on_step_change(move |v| {
                        if accept.get() {
                            active.revise(|old| *old = v)
                        }
                    })
                    .steps([
                        sapid::step("details").label("Details"),
                        sapid::step("confirm").label("Confirm"),
                        sapid::step("finish").label("Finish"),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept step changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow skipping")
                    .checked(skip)
                    .on_checked_change(move |v| skip.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable workflow")
                    .checked(enabled)
                    .on_checked_change(move |v| enabled.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(active.map(|v| format!("Accepted step: {}", v + 1))),
            ))
            .show_in(scope)
    })
}
