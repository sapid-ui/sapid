use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let outer = scope.cage(false);
        let inner = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Start project editing")
                    .on_press(move || outer.revise(|s| *s = true)),
                sapid::focus_scope()
                    .active(outer)
                    .loop_tab(true)
                    .contain_focus(true)
                    .auto_focus(true)
                    .restore_focus(true)
                    .style(
                        Style::new()
                            .display(glory::style::Display::Flex)
                            .flex_direction(glory::style::FlexDirection::Column)
                            .gap(12.),
                    )
                    .fill((
                        sapid::field("focus-outer-draft")
                            .theme(theme.clone())
                            .label("Project editing draft")
                            .control(|field| sapid::text_field().field(field)),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Edit nested note")
                            .on_press(move || inner.revise(|s| *s = true)),
                        sapid::focus_scope()
                            .active(inner)
                            .loop_tab(true)
                            .contain_focus(true)
                            .auto_focus(true)
                            .restore_focus(true)
                            .style(
                                Style::new()
                                    .display(glory::style::Display::Flex)
                                    .flex_direction(glory::style::FlexDirection::Column)
                                    .gap(10.),
                            )
                            .fill((
                                sapid::field("focus-inner-draft")
                                    .theme(theme.clone())
                                    .label("Nested project note")
                                    .control(|field| sapid::text_field().field(field)),
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Finish nested note")
                                    .on_press(move || inner.revise(|s| *s = false)),
                            )),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Finish project editing")
                            .on_press(move || outer.revise(|s| *s = false)),
                    )),
            ))
            .show_in(scope)
    })
}
