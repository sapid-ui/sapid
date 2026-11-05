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
                sapid::field("tags-controlled-field")
                    .theme(theme.clone())
                    .label("Controlled keywords")
                    .control(move |field| {
                        sapid::tags_input("tags-controlled")
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
                    .label("Accept keywords proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(state.map(|v| format!("Accepted keywords: {}", v.values.join(", ")))),
            ))
            .show_in(scope)
    })
}
