use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(sapid::TagsInputState {
            values: vec![
                "Design".into(),
                "Accessibility".into(),
                "Localization and multilingual text editing".into(),
                "Testing".into(),
            ],
            search: String::new(),
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("pills-wrapping-field")
                    .theme(theme.clone())
                    .label("Project topics")
                    .description("The hint appears only when all topics have been removed.")
                    .control(move |field| {
                        sapid::pills_input("pills-wrapping")
                            .field(field)
                            .state(state)
                            .on_change(move |v| state.revise(|old| *old = v))
                            .placeholder(state.map(|v| {
                                if v.values.is_empty() {
                                    "Add a project topic"
                                } else {
                                    ""
                                }
                                .to_owned()
                            }))
                            .clearable(true)
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Use two short topics")
                    .on_press(move || {
                        state.revise(|v| v.values = vec!["Design".into(), "Testing".into()])
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore many topics")
                    .on_press(move || {
                        state.revise(|v| {
                            v.values = vec![
                                "Design".into(),
                                "Accessibility".into(),
                                "Localization and multilingual text editing".into(),
                                "Testing".into(),
                            ]
                        })
                    }),
            ))
            .show_in(scope)
    })
}
