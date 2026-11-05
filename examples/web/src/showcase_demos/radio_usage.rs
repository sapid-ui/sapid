use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("delivery-field")
        .theme(theme)
        .label("Delivery method")
        .description("Choose one delivery option.")
        .control(|field| {
            sapid::radio_group("delivery-options")
                .field(field)
                .default_value("standard")
                .items([
                    sapid::radio("standard").label("Standard delivery"),
                    sapid::radio("express").label("Express delivery"),
                    sapid::radio("collection")
                        .label("Collection")
                        .enabled(false),
                ])
        })
}
