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
                    .label("Loading contacts")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::autocomplete("autocomplete-load-left")
                    .theme(theme.clone())
                    .label("Left loading indicator")
                    .loading(loading)
                    .loading_position(sapid::AutocompleteLoadingPosition::Left)
                    .options([
                        sapid::autocomplete_option("ana", "ana@example.com"),
                        sapid::autocomplete_option("sam", "sam@example.com"),
                        sapid::autocomplete_option("lee", "lee@example.com"),
                    ]),
                sapid::autocomplete("autocomplete-load-right")
                    .theme(theme.clone())
                    .label("Right loading indicator")
                    .loading(loading)
                    .options([
                        sapid::autocomplete_option("ana", "ana@example.com"),
                        sapid::autocomplete_option("sam", "sam@example.com"),
                        sapid::autocomplete_option("lee", "lee@example.com"),
                    ]),
            ))
            .show_in(scope)
    })
}
