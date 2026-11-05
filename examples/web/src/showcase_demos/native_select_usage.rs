use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("native-country")
        .theme(theme)
        .label("Country")
        .control(|field| {
            sapid::native_select("native-country-control")
                .field(field)
                .placeholder("Choose a country")
                .option(sapid::select_option("fr", "France"))
                .option(sapid::select_option("de", "Germany"))
                .option(sapid::select_option("jp", "Japan"))
        })
}
