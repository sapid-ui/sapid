use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-autocomplete")
        .theme(theme.clone())
        .label("Contact email")
        .control(move |field| {
            sapid::autocomplete("contact-email")
                .field(field)
                .theme(theme.clone())
                .placeholder("Search or enter an address")
                .clearable(true)
                .options([
                    sapid::autocomplete_option("ana", "ana@example.com"),
                    sapid::autocomplete_option("sam", "sam@example.com"),
                    sapid::autocomplete_option("lee", "lee@example.com"),
                ])
                .option_content(
                    "ana",
                    sapid::SelectOptionContent::default().description("Design team"),
                )
        })
}
