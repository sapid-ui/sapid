use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("mask-phone")
        .theme(theme)
        .label("Phone number")
        .description("Include the area code.")
        .control(|field| {
            sapid::mask_input(sapid::MaskPattern::parse("(###) ###-####").unwrap())
                .field(field)
                .placeholder("(000) 000-0000")
                .style(Style::new().width(Length::Percent(100.)))
        })
}
