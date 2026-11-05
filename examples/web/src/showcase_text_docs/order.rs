use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("input-order")
        .theme(theme)
        .label("Public identifier")
        .description("This description appears before the label.")
        .error("Choose another identifier.")
        .order([
            sapid::FieldSlot::Description,
            sapid::FieldSlot::Label,
            sapid::FieldSlot::Control,
            sapid::FieldSlot::Error,
        ])
        .control(|field| sapid::text_field().field(field).default_value("taken-name"))
}
