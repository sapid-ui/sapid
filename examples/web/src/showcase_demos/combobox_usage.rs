use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-combobox")
        .theme(theme.clone())
        .label("Framework")
        .control(move |field| {
            sapid::combobox("framework-combobox")
                .field(field)
                .theme(theme.clone())
                .placeholder("Search frameworks")
                .options([
                    sapid::select_option("glory", "Glory"),
                    sapid::select_option("dioxus", "Dioxus"),
                    sapid::select_option("leptos", "Leptos"),
                ])
                .option_content(
                    "glory",
                    sapid::SelectOptionContent::default().description("Rust user interfaces"),
                )
        })
}
