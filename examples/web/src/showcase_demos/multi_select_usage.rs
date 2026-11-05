use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-multi-select")
        .theme(theme.clone())
        .label("Technology skills")
        .control(move |field| {
            sapid::multi_select("technology-skills")
                .field(field)
                .theme(theme.clone())
                .placeholder("Add a skill")
                .clearable(true)
                .max_values(3)
                .hide_picked_options(true)
                .default_value(vec!["rust".into(), "ui".into()])
                .options([
                    sapid::select_option("rust", "Rust"),
                    sapid::select_option("ui", "UI design"),
                    sapid::select_option("wasm", "WebAssembly"),
                    sapid::select_option("testing", "Testing"),
                ])
        })
}
