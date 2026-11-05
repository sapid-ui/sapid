use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("number-quantity")
        .theme(theme)
        .label("Quantity")
        .description("Choose between 1 and 99.")
        .control(|field| {
            sapid::number_input()
                .field(field)
                .default_value(sapid::Decimal::new(1, 0).unwrap())
                .min(Some(sapid::Decimal::new(1, 0).unwrap()))
                .max(Some(sapid::Decimal::new(99, 0).unwrap()))
        })
}
