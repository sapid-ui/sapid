use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::autocomplete("autocomplete-grouped")
            .theme(theme.clone())
            .label("Team contact")
            .default_value("ana@example.com")
            .clearable(true)
            .group(
                sapid::select_group("Design")
                    .option(sapid::autocomplete_option("ana", "ana@example.com")),
            )
            .group(
                sapid::select_group("Engineering")
                    .option(sapid::autocomplete_option("sam", "sam@example.com"))
                    .option(sapid::autocomplete_option("lee", "lee@example.com").disabled(true)),
            )
            .option_content(
                "ana",
                sapid::SelectOptionContent::default()
                    .leading("A")
                    .description("Product designer"),
            )
            .empty_message("No matching team contacts"),
        sapid::autocomplete("autocomplete-unavailable")
            .theme(theme.clone())
            .label("Unavailable contact")
            .enabled(false)
            .options([
                sapid::autocomplete_option("ana", "ana@example.com"),
                sapid::autocomplete_option("sam", "sam@example.com"),
                sapid::autocomplete_option("lee", "lee@example.com"),
            ]),
        sapid::autocomplete("autocomplete-readonly")
            .theme(theme)
            .label("Read-only contact")
            .default_value("sam@example.com")
            .readonly(true)
            .options([
                sapid::autocomplete_option("ana", "ana@example.com"),
                sapid::autocomplete_option("sam", "sam@example.com"),
                sapid::autocomplete_option("lee", "lee@example.com"),
            ]),
    ))
}
