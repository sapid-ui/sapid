use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::combobox("combo-groups")
        .theme(theme)
        .label("Grouped frameworks")
        .group(
            sapid::select_group("Rust")
                .option(sapid::select_option("glory", "Glory"))
                .option(sapid::select_option("dioxus", "Dioxus")),
        )
        .group(
            sapid::select_group("Other")
                .option(sapid::select_option("react", "React"))
                .option(sapid::select_option("vue", "Vue").disabled(true)),
        )
        .empty_message("No matching frameworks")
}
