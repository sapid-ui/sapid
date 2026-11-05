use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(Some("Projects".to_owned()));
        let content_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::floating_indicator("workspace-sections")
                    .theme(theme.clone())
                    .active(active)
                    .content(move |context| {
                        sapid::group().theme(content_theme.clone()).gap(12.).fill(
                            ["Projects", "Teams", "Activity"]
                                .into_iter()
                                .map(|key| {
                                    sapid::floating_indicator_target(context.clone(), key).fill(
                                        sapid::button()
                                            .theme(content_theme.clone())
                                            .variant(sapid::ButtonVariant::Subtle)
                                            .label(key)
                                            .on_press(move || {
                                                active.revise(|v| *v = Some(key.to_owned()))
                                            }),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    }),
                sapid::text().theme(theme.clone()).content(
                    active
                        .map(|v| format!("Workspace section: {}", v.as_deref().unwrap_or("none"))),
                ),
            ))
            .show_in(scope)
    })
}
