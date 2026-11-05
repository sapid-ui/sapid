use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::combobox("combo-prefix")
                    .theme(theme.clone())
                    .label("Prefix lookup")
                    .filter(sapid::ComboboxFilter::Prefix)
                    .limit(2)
                    .loading(loading)
                    .options([
                        sapid::select_option("glory", "Glory"),
                        sapid::select_option("dioxus", "Dioxus"),
                        sapid::select_option("leptos", "Leptos"),
                    ])
                    .empty_message("No prefix matches"),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show framework loading")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
