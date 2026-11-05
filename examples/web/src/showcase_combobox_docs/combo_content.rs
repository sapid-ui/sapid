use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::combobox("combo-content")
        .theme(theme)
        .label("Framework with descriptions")
        .options([
            sapid::select_option("glory", "Glory"),
            sapid::select_option("dioxus", "Dioxus"),
            sapid::select_option("leptos", "Leptos"),
        ])
        .option_content(
            "glory",
            sapid::SelectOptionContent::default()
                .leading("G")
                .description("Rust user interfaces")
                .trailing("Stable"),
        )
        .option_content(
            "dioxus",
            sapid::SelectOptionContent::default().description("Rust applications"),
        )
}
