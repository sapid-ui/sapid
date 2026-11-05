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
                    .label("Loading skill options")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::multi_select("multi-load-left")
                    .theme(theme.clone())
                    .label("Skills with left loading")
                    .loading(loading)
                    .loading_position(sapid::MultiSelectLoadingPosition::Left)
                    .default_value(vec!["rust".into()])
                    .clearable(true)
                    .options([
                        sapid::select_option("rust", "Rust"),
                        sapid::select_option("ui", "UI design"),
                        sapid::select_option("wasm", "WebAssembly"),
                        sapid::select_option("testing", "Testing"),
                    ]),
                sapid::multi_select("multi-load-right")
                    .theme(theme.clone())
                    .label("Skills with right loading")
                    .loading(loading)
                    .default_value(vec!["rust".into()])
                    .clearable(true)
                    .options([
                        sapid::select_option("rust", "Rust"),
                        sapid::select_option("ui", "UI design"),
                        sapid::select_option("wasm", "WebAssembly"),
                        sapid::select_option("testing", "Testing"),
                    ]),
            ))
            .show_in(scope)
    })
}
