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
                    .label("Loading topics")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::field("pills-left-field")
                    .theme(theme.clone())
                    .label("Topics with left loading")
                    .control(move |field| {
                        sapid::pills_input("pills-left")
                            .field(field)
                            .loading(loading)
                            .loading_position(sapid::TagsInputLoadingPosition::Left)
                            .default_value(vec!["Rust".into()])
                            .clearable(true)
                    }),
                sapid::field("pills-right-field")
                    .theme(theme.clone())
                    .label("Topics with right loading")
                    .control(move |field| {
                        sapid::pills_input("pills-right")
                            .field(field)
                            .loading(loading)
                            .default_value(vec!["Rust".into()])
                            .clearable(true)
                    }),
            ))
            .show_in(scope)
    })
}
