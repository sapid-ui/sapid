use glory::prelude::*;
fn decimal(value: &str) -> sapid::Decimal {
    sapid::Decimal::parse(value).unwrap()
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("number-english")
            .theme(theme.clone())
            .label("English grouped decimal")
            .control(|field| {
                sapid::number_input()
                    .field(field)
                    .default_value(decimal("1234.5"))
                    .grouping(true)
            }),
        sapid::field("number-german")
            .theme(theme.clone())
            .label("German grouped decimal")
            .control(|field| {
                sapid::number_input()
                    .field(field)
                    .default_value(decimal("1234.5"))
                    .locale(sapid::NumberLocale::DeDe)
                    .grouping(true)
            }),
        sapid::field("number-fine")
            .theme(theme)
            .label("Exact eighteen-digit step")
            .description(
                "Arithmetic retains 18 fractional digits; the accessibility coordinate is lossy.",
            )
            .control(|field| {
                sapid::number_input()
                    .field(field)
                    .default_value(decimal("0.000000000000000001"))
                    .step(decimal("0.000000000000000001"))
            }),
    ))
}
