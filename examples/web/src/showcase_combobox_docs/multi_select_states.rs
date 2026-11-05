use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::multi_select("multi-disabled")
            .theme(theme.clone())
            .label("Unavailable skills")
            .enabled(false)
            .default_value(vec!["rust".into()])
            .options([
                sapid::select_option("rust", "Rust"),
                sapid::select_option("ui", "UI design"),
                sapid::select_option("wasm", "WebAssembly"),
                sapid::select_option("testing", "Testing"),
            ]),
        sapid::multi_select("multi-readonly")
            .theme(theme.clone())
            .label("Read-only skills")
            .readonly(true)
            .default_value(vec!["rust".into()])
            .options([
                sapid::select_option("rust", "Rust"),
                sapid::select_option("ui", "UI design"),
                sapid::select_option("wasm", "WebAssembly"),
                sapid::select_option("testing", "Testing"),
            ]),
        sapid::multi_select("multi-invalid")
            .theme(theme.clone())
            .label("Invalid required skills")
            .invalid(true)
            .required(true)
            .options([
                sapid::select_option("rust", "Rust"),
                sapid::select_option("ui", "UI design"),
                sapid::select_option("wasm", "WebAssembly"),
                sapid::select_option("testing", "Testing"),
            ]),
        sapid::stack().theme(theme.clone()).gap(12.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .enumerate()
            .map(|(i, size)| {
                sapid::multi_select(format!("multi-size-{i}"))
                    .theme(theme.clone())
                    .size(size)
                    .label(format!("Skills size {}", i + 1))
                    .default_value(vec!["rust".into()])
                    .clearable(true)
                    .options([
                        sapid::select_option("rust", "Rust"),
                        sapid::select_option("ui", "UI design"),
                        sapid::select_option("wasm", "WebAssembly"),
                        sapid::select_option("testing", "Testing"),
                    ])
            })
            .collect::<Vec<_>>(),
        ),
    ))
}
