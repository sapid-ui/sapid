use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::field("radio-readonly")
            .theme(theme.clone())
            .label("Saved delivery")
            .description("Your administrator manages this choice.")
            .readonly(true)
            .control(|field| {
                sapid::radio_group("saved-delivery")
                    .field(field)
                    .default_value("standard")
                    .items([
                        sapid::radio("standard").label("Standard"),
                        sapid::radio("express").label("Express"),
                    ])
            }),
        sapid::field("radio-invalid")
            .theme(theme.clone())
            .label("Required delivery")
            .required(true)
            .error("Choose a delivery method.")
            .control(|field| {
                sapid::radio_group("required-delivery").field(field).items([
                    sapid::radio("standard").label("Standard"),
                    sapid::radio("express").label("Express"),
                ])
            }),
    ))
}
