use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::select("popover-groups")
        .theme(theme)
        .name("Framework category")
        .placeholder("Choose a framework")
        .group(
            sapid::select_group("Rust")
                .option(sapid::select_option("glory", "Glory"))
                .option(sapid::select_option("dioxus", "Dioxus")),
        )
        .group(
            sapid::select_group("JavaScript")
                .option(sapid::select_option("react", "React"))
                .option(sapid::select_option("vue", "Vue").disabled(true)),
        )
        .option_content(
            "glory",
            sapid::SelectOptionContent::default().description("Rust user interfaces"),
        )
        .max_dropdown_height(160.)
}
