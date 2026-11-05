use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<String>);
        let search = scope.cage(String::new());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::combobox("combo-controlled")
                    .theme(theme.clone())
                    .label("Controlled framework search")
                    .value(value)
                    .search(search)
                    .on_search_change(move |v| search.revise(|old| *old = v))
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v)
                        }
                    })
                    .options([
                        sapid::select_option("glory", "Glory"),
                        sapid::select_option("dioxus", "Dioxus"),
                        sapid::select_option("leptos", "Leptos"),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept selected framework")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.as_ref()
                        .map(|v| format!("Selected framework: {v}"))
                        .unwrap_or_else(|| "No framework selected".into())
                })),
            ))
            .show_in(scope)
    })
}
