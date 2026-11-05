use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-select-searchable")
        .theme(theme)
        .label("Searchable composition")
        .description("Focus the editor to filter the choices")
        .control(move |field| {
            sapid::combobox("showcase-select-searchable")
                .field(field)
                .default_search("a")
                .filter(sapid::ComboboxFilter::Prefix)
                .limit(2)
                .loading(true)
                .empty_message("Nothing found")
                .options([
                    sapid::select_option("angular", "Angular"),
                    sapid::select_option("astro", "Astro"),
                    sapid::select_option("alpine", "Alpine"),
                    sapid::select_option("ark", "Ark UI"),
                    sapid::select_option("react", "React"),
                ])
        })
}
