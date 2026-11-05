use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(24.).fill((
        workflow(theme.clone(), "retained", sapid::ContentPolicy::KeepMounted),
        workflow(theme, "retired", sapid::ContentPolicy::Unmount),
    ))
}
fn workflow(theme: sapid::Theme, key: &'static str, policy: sapid::ContentPolicy) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(0_usize);
        let content_theme = theme.clone();
        let completed_theme = theme.clone();
        let review_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::stepper(key)
                    .theme(theme.clone())
                    .name(format!("{key} content workflow"))
                    .content_policy(policy)
                    .active(active)
                    .selectable(true)
                    .on_step_change(move |v| active.revise(|old| *old = v))
                    .steps([
                        sapid::step("draft").label("Draft").content(move || {
                            widget({
                                let theme = content_theme.clone();
                                move |scope| {
                                    let draft = scope.cage(String::new());
                                    sapid::text_field()
                                        .theme(theme.clone())
                                        .name(format!("{key} project draft"))
                                        .placeholder("Write a project note")
                                        .value(draft)
                                        .on_value_change(move |v| draft.revise(|old| *old = v))
                                        .show_in(scope)
                                }
                            })
                        }),
                        sapid::step("review").label("Review").content(move || {
                            sapid::text()
                                .theme(review_theme.clone())
                                .content("Review the project before completing the workflow.")
                        }),
                    ])
                    .completed(move || {
                        sapid::text()
                            .theme(completed_theme.clone())
                            .content("Project setup completed")
                    }),
                sapid::group().theme(theme.clone()).gap(8.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label(format!("Back in {key} workflow"))
                        .enabled(active.map(|v| *v > 0))
                        .on_press(move || active.revise(|v| *v = v.saturating_sub(1))),
                    sapid::button()
                        .theme(theme.clone())
                        .label(format!("Next in {key} workflow"))
                        .enabled(active.map(|v| *v < 2))
                        .on_press(move || active.revise(|v| *v = (*v + 1).min(2))),
                )),
            ))
            .show_in(scope)
    })
}
