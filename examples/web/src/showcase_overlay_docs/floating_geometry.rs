use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(Some("projects".to_owned()));
        let long = scope.cage(false);
        let reverse = scope.cage(false);
        let order = scope.cage(vec!["projects".to_owned(), "teams".to_owned()]);
        let revision = scope.cage(0u64);
        let body = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use longer project label")
                    .checked(long)
                    .on_checked_change(move |v| long.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Reverse project controls")
                    .checked(reverse)
                    .on_checked_change(move |v| {
                        reverse.revise(|old| *old = v);
                        order.revise(|keys| keys.reverse());
                        revision.revise(|v| *v += 1);
                    }),
                sapid::floating_indicator("resize-indicator")
                    .theme(theme.clone())
                    .active(active)
                    .measurement_revision(revision)
                    .content(move |context| {
                        let body = body.clone();
                        sapid::group().theme(body.clone()).gap(12.).fill(
                            each(order).key(|key| key.clone()).children(move |entry| {
                                let context = context.clone();
                                let body = body.clone();
                                widget(move |scope| {
                                    let key = entry.value.get();
                                    let next = key.clone();
                                    let label: Vary<String> = if key == "projects" {
                                        long.map(|v| {
                                            if *v {
                                                "All recent projects".to_owned()
                                            } else {
                                                "Projects".to_owned()
                                            }
                                        })
                                        .into()
                                    } else {
                                        "Teams".to_owned().into()
                                    };
                                    sapid::floating_indicator_target(context.clone(), key)
                                        .fill(
                                            sapid::button()
                                                .theme(body.clone())
                                                .label(label)
                                                .on_press(move || {
                                                    active.revise(|v| *v = Some(next.clone()))
                                                }),
                                        )
                                        .show_in(scope)
                                })
                            }),
                        )
                    }),
            ))
            .show_in(scope)
    })
}
