use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "workspace-guide")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let draft = scope.cage("Team workspace".to_owned());
        let created = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Start workspace tour")
                    .on_press(move || {
                        current.revise(|v| *v = 0);
                        open.revise(|v| *v = true);
                    }),
                sapid::tour_target(&context, "workspace-name").fill(
                    sapid::text_field()
                        .theme(theme.clone())
                        .name("Tour workspace name")
                        .value(draft)
                        .on_value_change(move |v| draft.revise(|old| *old = v)),
                ),
                sapid::tour_target(&context, "create-project").fill(
                    sapid::button()
                        .theme(theme.clone())
                        .label("Create guided project")
                        .on_press(move || created.revise(|v| *v = true)),
                ),
                sapid::text().theme(theme.clone()).content(created.map(|v| {
                    if *v {
                        "Your guided project is ready.".to_owned()
                    } else {
                        "Name your workspace, then create a project.".to_owned()
                    }
                })),
                sapid::tour(&context, "getting-started")
                    .theme(theme.clone())
                    .open(open)
                    .current(current)
                    .steps([
                        sapid::tour_step(
                            "workspace-name",
                            "Name your workspace",
                            "Choose a name your team will recognize.",
                        ),
                        sapid::tour_step(
                            "create-project",
                            "Create your first project",
                            "Use this action to start organizing your work.",
                        ),
                    ])
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .on_current_change(move |v| current.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
