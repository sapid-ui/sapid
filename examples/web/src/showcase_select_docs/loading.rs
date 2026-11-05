use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-select-loading")
        .theme(theme)
        .label("Loading options")
        .description("The control exposes busy state and remains a real select combobox.")
        .control(move |field| {
            sapid::select("showcase-select-loading")
                .field(field)
                .loading(true)
                .option(sapid::select_option("pending", "Pending option"))
        })
}
