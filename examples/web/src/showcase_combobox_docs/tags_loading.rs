use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Loading keywords")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::field("tags-left-field")
                    .theme(theme.clone())
                    .label("Keywords with left loading")
                    .control(move |field| {
                        sapid::tags_input("tags-left")
                            .field(field)
                            .loading(loading)
                            .loading_position(sapid::TagsInputLoadingPosition::Left)
                            .default_value(vec!["Rust".into()])
                            .clearable(true)
                    }),
                sapid::field("tags-right-field")
                    .theme(theme.clone())
                    .label("Keywords with right loading")
                    .control(move |field| {
                        sapid::tags_input("tags-right")
                            .field(field)
                            .loading(loading)
                            .default_value(vec!["Rust".into()])
                            .clearable(true)
                    }),
            ))
            .show_in(scope)
    })
}
