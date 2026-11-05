use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let single = scope.cage("rust".to_owned());
        let multiple = scope.cage(vec!["rust".to_owned()]);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::chip_group("single-language")
                    .theme(theme.clone())
                    .label("One language")
                    .value(single)
                    .items([
                        sapid::chip_item("rust", "Rust"),
                        sapid::chip_item("wasm", "Wasm"),
                    ])
                    .on_value_change(move |v| single.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Clear single selection")
                    .on_press(move || single.revise(String::clear)),
                sapid::text().theme(theme.clone()).content(single.map(|v| {
                    if v.is_empty() {
                        "Single: none".to_owned()
                    } else {
                        format!("Single: {v}")
                    }
                })),
                sapid::chip_group("multiple-languages")
                    .theme(theme.clone())
                    .label("Several languages")
                    .mode(sapid::ChipMode::Multiple)
                    .values(multiple)
                    .items([
                        sapid::chip_item("rust", "Rust"),
                        sapid::chip_item("wasm", "Wasm"),
                    ])
                    .on_values_change(move |v| multiple.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(multiple.map(|v| format!("Multiple: {}", v.join(", ")))),
            ))
            .show_in(scope)
    })
}
