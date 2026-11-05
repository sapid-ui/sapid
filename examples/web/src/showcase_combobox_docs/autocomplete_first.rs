use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::autocomplete("autocomplete-first")
        .theme(theme)
        .label("First matching contact")
        .placeholder("Type an address, then press Enter")
        .select_first_option_on_change(true)
        .options([
            sapid::autocomplete_option("ana", "ana@example.com"),
            sapid::autocomplete_option("sam", "sam@example.com"),
            sapid::autocomplete_option("lee", "lee@example.com"),
        ])
}
