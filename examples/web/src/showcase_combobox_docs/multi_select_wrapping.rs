use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let values = scope.cage(vec![
            "long".to_owned(),
            "rust".into(),
            "ui".into(),
            "wasm".into(),
            "testing".into(),
        ]);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::multi_select("multi-wrapping")
                    .theme(theme.clone())
                    .label("Project skills")
                    .value(values)
                    .on_value_change(move |v| values.revise(|old| *old = v))
                    .placeholder("Add another skill")
                    .clearable(true)
                    .options([
                        sapid::select_option("rust", "Rust"),
                        sapid::select_option("ui", "UI design"),
                        sapid::select_option("wasm", "WebAssembly"),
                        sapid::select_option("testing", "Testing"),
                    ])
                    .option(sapid::select_option(
                        "long",
                        "Interface architecture, accessibility and multilingual application design",
                    )),
                sapid::button()
                    .theme(theme.clone())
                    .label("Use two short skills")
                    .on_press(move || values.revise(|v| *v = vec!["rust".into(), "ui".into()])),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore many skills")
                    .on_press(move || {
                        values.revise(|v| {
                            *v = vec![
                                "long".into(),
                                "rust".into(),
                                "ui".into(),
                                "wasm".into(),
                                "testing".into(),
                            ]
                        })
                    }),
            ))
            .show_in(scope)
    })
}
