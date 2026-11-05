use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("mask-editing").theme(theme).label("Edit the reference")
        .description("Select digits to replace them, paste 123456 or 12-34-56, or delete next to a separator.")
        .control(|field|sapid::mask_input(sapid::MaskPattern::parse("##-##-##").unwrap()).field(field)
            .default_value("123456").placeholder("00-00-00"))
}
