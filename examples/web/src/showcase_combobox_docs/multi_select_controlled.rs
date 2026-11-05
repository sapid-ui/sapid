use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let values = scope.cage(vec!["rust".to_owned()]);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::multi_select("multi-controlled")
                    .theme(theme.clone())
                    .label("Controlled skills")
                    .value(values)
                    .on_value_change(move |v| {
                        if accept.get() {
                            values.revise(|old| *old = v)
                        }
                    })
                    .clearable(true)
                    .dropdown_label("Open controlled skill options")
                    .options([
                        sapid::select_option("rust", "Rust"),
                        sapid::select_option("ui", "UI design"),
                        sapid::select_option("wasm", "WebAssembly"),
                        sapid::select_option("testing", "Testing"),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept skill proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(values.map(|v| format!("Accepted skills: {}", v.join(", ")))),
            ))
            .show_in(scope)
    })
}
