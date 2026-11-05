use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-color-state-error")
        .theme(theme.clone())
        .label("Error")
        .error("Choose an approved color")
        .control(|field| sapid::color_input("showcase-color-state-error").field(field))
}
