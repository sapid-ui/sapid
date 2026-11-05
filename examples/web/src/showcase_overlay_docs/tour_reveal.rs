use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "scrolling-guide")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Start scrolling guide")
                    .on_press(move || {
                        current.revise(|v| *v = 0);
                        open.revise(|v| *v = true);
                    }),
                view()
                    .class("tour-local-scroll")
                    .style(
                        Style::new()
                            .height(Length::Px(220.))
                            .overflow_y(Overflow::Auto),
                    )
                    .fill(
                        sapid::stack().theme(theme.clone()).gap(12.).fill((
                            sapid::tour_target(&context, "name").fill(
                                sapid::text_field()
                                    .theme(theme.clone())
                                    .name("Scrolling tour draft")
                                    .default_value("Retain guide draft"),
                            ),
                            view().style(Style::new().height(Length::Px(420.))).fill(
                                sapid::text()
                                    .theme(theme.clone())
                                    .content("Project content continues below."),
                            ),
                            sapid::tour_target(&context, "publish").fill(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Publish guided draft"),
                            ),
                        )),
                    ),
                sapid::tour(&context, "scrolling-tour")
                    .theme(theme.clone())
                    .open(open)
                    .current(current)
                    .steps([
                        sapid::tour_step("name", "Draft name", "Start with your retained draft."),
                        sapid::tour_step(
                            "publish",
                            "Publish action",
                            "The local viewport reveals this action.",
                        ),
                    ])
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .on_current_change(move |v| current.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
