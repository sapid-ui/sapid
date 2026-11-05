use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::multi_select("multi-grouped")
        .theme(theme)
        .label("Grouped skills")
        .default_value(vec!["rust".into()])
        .clearable(true)
        .max_values(2)
        .hide_picked_options(true)
        .limit(3)
        .group(
            sapid::select_group("Languages")
                .option(sapid::select_option("rust", "Rust"))
                .option(sapid::select_option("go", "Go")),
        )
        .group(
            sapid::select_group("Tools")
                .option(sapid::select_option("ui", "UI design"))
                .option(sapid::select_option("testing", "Testing"))
                .option(sapid::select_option("legacy", "Legacy tool").disabled(true)),
        )
        .option_content(
            "testing",
            sapid::SelectOptionContent::default().description("Quality assurance"),
        )
        .empty_message("No more matching skills")
}
