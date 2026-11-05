use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "recoverable-guide")?;
        let open = scope.cage(false);
        let present = scope.cage(true);
        let ctx = context.clone();
        let body = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Keep tour target mounted")
                    .checked(present)
                    .on_checked_change(move |v| present.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Start recoverable tour")
                    .on_press(move || open.revise(|v| *v = true)),
                when(present, move || {
                    sapid::tour_target(&ctx, "project").fill(
                        sapid::button()
                            .theme(body.clone())
                            .label("Recoverable project target"),
                    )
                }),
                sapid::tour(&context, "recoverable-tour")
                    .theme(theme.clone())
                    .open(open)
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .step(sapid::tour_step(
                        "project",
                        "Current project",
                        "Temporarily remove and restore this target with the checkbox.",
                    )),
            ))
            .show_in(scope)
    })
}
