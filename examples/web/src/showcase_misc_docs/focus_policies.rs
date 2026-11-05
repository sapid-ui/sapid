use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(false);
        let looping = scope.cage(true);
        let contain = scope.cage(true);
        let restore = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Start policy form")
                    .on_press(move || active.revise(|s| *s = true)),
                sapid::focus_scope()
                    .configuration(scope.bond(move || glory::render::FocusScopeConfig {
                        active: active.get(),
                        loop_tab: looping.get(),
                        contain_focus: contain.get(),
                        auto_focus: true,
                        restore_focus: restore.get(),
                        ..Default::default()
                    }))
                    .style(
                        Style::new()
                            .display(glory::style::Display::Flex)
                            .flex_direction(glory::style::FlexDirection::Column)
                            .gap(12.),
                    )
                    .fill((
                        sapid::field("focus-policy-note")
                            .theme(theme.clone())
                            .label("Policy form note")
                            .control(|field| {
                                sapid::text_field()
                                    .field(field)
                                    .default_value("Keep my current note")
                            }),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Loop Tab inside form")
                            .checked(looping)
                            .on_checked_change(move |v| looping.revise(|s| *s = v)),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Contain focus inside form")
                            .checked(contain)
                            .on_checked_change(move |v| contain.revise(|s| *s = v)),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Restore invoking focus")
                            .checked(restore)
                            .on_checked_change(move |v| restore.revise(|s| *s = v)),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Finish policy form")
                            .on_press(move || active.revise(|s| *s = false)),
                    )),
            ))
            .show_in(scope)
    })
}
