use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "controlled-guide")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let accept = scope.cage(false);
        let close = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow tour step changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow tour dismissal")
                    .checked(close)
                    .on_checked_change(move |v| close.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Start controlled tour")
                    .on_press(move || {
                        current.revise(|v| *v = 0);
                        open.revise(|v| *v = true);
                    }),
                sapid::tour_target(&context, "draft").fill(
                    sapid::text_field()
                        .theme(theme.clone())
                        .name("Controlled tour draft")
                        .default_value("Keep this draft"),
                ),
                sapid::tour_target(&context, "review").fill(
                    sapid::button()
                        .theme(theme.clone())
                        .label("Review guided project"),
                ),
                sapid::tour(&context, "controlled-tour")
                    .theme(theme.clone())
                    .open(open)
                    .current(current)
                    .steps([
                        sapid::tour_step("draft", "Draft step", "Edit your project draft."),
                        sapid::tour_step("review", "Review step", "Review the prepared project."),
                    ])
                    .on_current_change(move |v| {
                        if accept.get() {
                            current.revise(|old| *old = v)
                        }
                    })
                    .on_open_change(move |v| {
                        if v || close.get() {
                            open.revise(|old| *old = v)
                        }
                    }),
            ))
            .show_in(scope)
    })
}
