use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(sapid::TagsInputState {
            values: vec!["Rust".into()],
            search: String::new(),
        });
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("pills-controlled-field")
                    .theme(theme.clone())
                    .label("Controlled topics")
                    .control(move |field| {
                        sapid::pills_input("pills-controlled")
                            .field(field)
                            .state(state)
                            .on_change(move |v| {
                                if accept.get() {
                                    state.revise(|old| *old = v)
                                }
                            })
                            .clearable(true)
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept topics proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(state.map(|v| format!("Accepted topics: {}", v.values.join(", ")))),
            ))
            .show_in(scope)
    })
}
