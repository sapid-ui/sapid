use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let vertical = scope.cage(false);
        let active = scope.cage(Some("Overview".to_owned()));
        let revision = scope.cage(0u64);
        let body = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use vertical workspace controls")
                    .checked(vertical)
                    .on_checked_change(move |v| {
                        vertical.revise(|old| *old = v);
                        revision.revise(|n| *n += 1);
                    }),
                sapid::floating_indicator("layout-indicator")
                    .theme(theme.clone())
                    .active(active)
                    .measurement_revision(revision)
                    .content(move |context| {
                        view()
                            .style(vertical.map(|v| {
                                Style::new()
                                    .display(Display::Flex)
                                    .flex_wrap(glory::style::FlexWrap::Wrap)
                                    .flex_direction(if *v {
                                        FlexDirection::Column
                                    } else {
                                        FlexDirection::Row
                                    })
                                    .gap(12.)
                            }))
                            .fill(
                                ["Overview", "Members", "Preferences"]
                                    .into_iter()
                                    .map(|key| {
                                        sapid::floating_indicator_target(context.clone(), key).fill(
                                            sapid::button()
                                                .theme(body.clone())
                                                .label(key)
                                                .on_press(move || {
                                                    active.revise(|v| *v = Some(key.to_owned()))
                                                }),
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                            )
                    }),
            ))
            .show_in(scope)
    })
}
