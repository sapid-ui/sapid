use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("native-groups")
        .theme(theme)
        .label("Grouped country")
        .control(|field| {
            sapid::native_select("native-groups-control")
                .field(field)
                .placeholder("Choose a country")
                .group(
                    sapid::select_group("Europe")
                        .option(sapid::select_option("fr", "France"))
                        .option(sapid::select_option("de", "Germany").disabled(true)),
                )
                .group(sapid::select_group("Asia").option(sapid::select_option("jp", "Japan")))
                .group(
                    sapid::select_group("Unavailable region")
                        .disabled(true)
                        .option(sapid::select_option("xx", "Unavailable country")),
                )
        })
}
