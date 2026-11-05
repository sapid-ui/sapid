use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-select-geometry")
        .theme(theme)
        .label("Bounded dropdown geometry")
        .control(move |field| {
            sapid::select("showcase-select-geometry")
                .field(field)
                .max_dropdown_height(180.)
                .default_value(Some("option-1".into()))
                .popup_placement(glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Top,
                    glory::render::AnchorAlign::End,
                ))
                .options((1..=12).map(|index| {
                    sapid::select_option(format!("option-{index}"), if index == 1 { "A very long selected option label that must remain inside the field at narrow viewport widths".to_owned() } else { format!("Option {index}") })
                }))
        })
}
