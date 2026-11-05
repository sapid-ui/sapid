use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("mask-pattern-phone")
            .theme(theme.clone())
            .label("Telephone pattern")
            .control(|field| {
                sapid::mask_input(sapid::MaskPattern::parse("(###) ###-####").unwrap())
                    .field(field)
                    .placeholder("(000) 000-0000")
                    .default_value("5551234567")
            }),
        sapid::field("mask-pattern-date")
            .theme(theme.clone())
            .label("Date-shaped reference")
            .description("A character pattern, without calendar validation.")
            .control(|field| {
                sapid::mask_input(sapid::MaskPattern::parse("####-##-##").unwrap())
                    .field(field)
                    .placeholder("YYYY-MM-DD")
                    .default_value("20261105")
            }),
        sapid::field("mask-pattern-code")
            .theme(theme)
            .label("Letter and number code")
            .control(|field| {
                sapid::mask_input(sapid::MaskPattern::parse("AA-####").unwrap())
                    .field(field)
                    .placeholder("AB-1234")
                    .default_value("AB1234")
            }),
    ))
}
