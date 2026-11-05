use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::autocomplete("autocomplete-blur")
            .theme(theme.clone())
            .label("Complete the first match on blur")
            .auto_select_on_blur(true)
            .options([
                sapid::autocomplete_option("ana", "ana@example.com"),
                sapid::autocomplete_option("sam", "sam@example.com"),
                sapid::autocomplete_option("lee", "lee@example.com"),
            ]),
        sapid::button()
            .theme(theme)
            .label("Move focus out of contacts"),
    ))
}
