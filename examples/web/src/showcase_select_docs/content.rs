use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-select-content")
        .theme(theme)
        .label("Structured option content")
        .control(move |field| {
            sapid::select("showcase-select-content")
                .field(field)
                .default_value(Some("paris".into()))
                .check_position(sapid::SelectCheckPosition::Right)
                .group(sapid::select_group("Europe").options([
                    sapid::select_option("paris", "Paris"),
                    sapid::select_option("berlin", "Berlin"),
                ]))
                .option_content(
                    "paris",
                    sapid::SelectOptionContent::default()
                        .leading("FR")
                        .description("Capital of France")
                        .trailing("UTC+1"),
                )
                .option_content(
                    "berlin",
                    sapid::SelectOptionContent::default()
                        .leading("DE")
                        .description("Capital of Germany")
                        .trailing("UTC+1"),
                )
        })
}
