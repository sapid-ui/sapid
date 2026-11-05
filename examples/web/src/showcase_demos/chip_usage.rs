use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::chip_group("languages")
        .theme(theme)
        .label("Languages")
        .mode(sapid::ChipMode::Multiple)
        .default_values(["rust".to_owned()])
        .items([
            sapid::chip_item("rust", "Rust"),
            sapid::chip_item("wasm", "Wasm"),
            sapid::chip_item("typescript", "TypeScript"),
        ])
}
